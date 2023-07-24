//! Structs define a group of formulas
//!
//! A [`Formula`] look like this
//! ```unknown
//! 加强铁板 = 6*铁板 + 12*螺丝 12s
//! ```
//! It means that 6 iron plates and 12 screws can be made into 1 reinforced iron plate in 12 seconds. 
//! The [`FormulaList`] is a group of formulas separated by new line. 
//! Note that the [`Formula`] and [`Item`] are wrapped in [`Rc`] to avoid duplication.
//!
//! The [ABNF](https://www.rfc-editor.org/rfc/rfc5234.txt) of the formula shows here:
//! ```abnf
//! formula_list   = formula *( new_line formula ) [new_line]
//! formula        = formula_target [*SP "=" *SP formula_source ] 1*SP speed
//! formula_target = formula_item
//! formula_source = formula_item *( *SP "+" *SP formula_item)
//! formula_item   = [amount "*"] item
//! amount         = 1*DIGIT
//! item           = ALPHA 1*( ALPHA / DIGIT )    ; consider ALPHA as Alphabetic in unicode
//! speed          = 1*DIGIT "s"    ; "s" is short for seconds
//! new_line       = LF / CRLF
//! ```

#[cfg(test)]
mod test;

use std::{
    fmt::{Debug, Display, Formatter},
    io::{BufRead, BufReader, Read},
    rc::Rc,
};

use crate::{Error, Fraction, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct FormulaList {
    items: Vec<Rc<Item>>,
    formulas: Vec<Rc<Formula>>,
}

impl FormulaList {
    pub fn formulas<'a>(&'a self) -> &[Rc<Formula>] {
        &self.formulas
    }

    pub fn from_reader(source: impl Read) -> Result<Self> {
        let mut reader = ParserReader::new(source);
        <FormulaList as Parsed>::parse(&mut reader)
    }

    pub fn get_item(&self, name: &str) -> Rc<Item> {
        for v in self.items.iter() {
            if v.name == name {
                return v.clone();
            }
        }
        todo!("not found")
    }
}

impl Parsed for FormulaList {
    fn first(c: char) -> bool {
        <Formula as Parsed>::first(c)
    }

    fn parse(reader: &mut impl ParserRead) -> Result<FormulaList> {
        let mut formulas: Vec<Rc<Formula>> = Vec::new();

        let formula = <Formula as Parsed>::parse(reader)?;
        formulas.push(Rc::new(formula));

        while <NewLine as Parsed>::first(reader.peek()?) {
            _ = <NewLine as Parsed>::parse(reader)?;

            if reader.peek()? == '\0' {
                break;
            }

            let formula = <Formula as Parsed>::parse(reader)?;
            formulas.push(Rc::new(formula));
        }

        Ok(FormulaList {
            items: reader.ctx().items.clone(), // todo
            formulas,
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Formula {
    target: FormulaItem,
    source: Vec<FormulaItem>,
    speed: Fraction,
}

impl Formula {
    pub fn target(&self) -> &FormulaItem {
        &self.target
    }

    pub fn source(&self) -> &[FormulaItem] {
        &self.source
    }

    pub fn speed(&self) -> Fraction {
        self.speed
    }

    fn fmt(&self, f: &mut Formatter<'_>, times: Fraction) -> std::fmt::Result {
        let FormulaItem(target, amount) = &self.target;
        let amount = times * amount.clone().into();
        write!(f, "{target:?}*{amount:?} =")?;
        for (i, v) in self.source.iter().enumerate() {
            if i != 0 {
                write!(f, " +")?;
            }
            let FormulaItem(source, amount) = v;
            let amount = times * amount.clone().into();
            write!(f, " {source:?}*{amount:?}")?;
        }
        if times == 1.into() {
            let speed = self.speed;
            write!(f, " {speed:?}/min")?;
        }
        Ok(())
    }
}

impl Debug for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Formula::fmt(self, f, 1.into())
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Formula::fmt(self, f, 1.into())
    }
}

impl Parsed for Formula {
    fn first(c: char) -> bool {
        <FormulaItem as Parsed>::first(c)
    }

    fn parse(reader: &mut impl ParserRead) -> Result<Formula> {
        let target = <FormulaItem as Parsed>::parse(reader)?;
        let mut source: Vec<FormulaItem> = Vec::new();

        reader.skip_space()?;
        if reader.peek()? == '=' {
            _ = reader.read()?;
            reader.skip_space()?;
            let c = reader.peek()?;
            if !<FormulaItem as Parsed>::first(c) {
                err_unknown_char(reader, "Formula-FormulaItem".to_string())?; // todo nameof()
            }
            let item = <FormulaItem as Parsed>::parse(reader)?;
            source.push(item);

            loop {
                reader.skip_space()?;
                if reader.peek()? != '+' {
                    break;
                }
                _ = reader.read()?;
                reader.skip_space()?;

                let c = reader.peek()?;
                if !<FormulaItem as Parsed>::first(c) {
                    todo!("unknown char")
                }
                let item = <FormulaItem as Parsed>::parse(reader)?;
                source.push(item);
            }
        }

        if !<Speed as Parsed>::first(reader.peek()?) {
            err_unknown_char(reader, "Formula-Speed".to_string())?;
        }
        let speed = <Speed as Parsed>::parse(reader)?.0;

        Ok(Formula {
            target,
            source,
            speed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaItem(Rc<Item>, u32);

impl FormulaItem {
    pub fn item(&self) -> Rc<Item> {
        self.0.clone()
    }
    pub fn amount(&self) -> u32 {
        self.1
    }
}

impl Parsed for FormulaItem {
    fn first(c: char) -> bool {
        c.is_alphabetic() || c.is_digit(10)
    }

    fn parse(reader: &mut impl ParserRead) -> Result<Self> {
        let mut amount = 1;
        let mut name = String::new();

        if reader.peek()?.is_digit(10) {
            amount = reader.read()?.to_digit(10).unwrap();
            while reader.peek()?.is_digit(10) {
                amount = amount * 10 + reader.read()?.to_digit(10).unwrap();
            }
            if reader.peek()? == '*' {
                _ = reader.read()?;
            } else {
                todo!("unknown char")
            }
        }

        if !reader.peek()?.is_alphabetic() {
            todo!("unknown char")
        }

        name.push(reader.read()?);
        while reader.peek()?.is_alphanumeric() {
            name.push(reader.read()?);
        }

        let item = reader.ctx().items.iter().find(|x| x.name == name);
        let item = match item {
            Some(v) => v.clone(),
            None => {
                let v = Rc::new(Item { name });
                reader.ctx().items.push(v.clone());
                v
            }
        };

        Ok(FormulaItem(item, amount))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Item {
    name: String,
}

impl Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = &self.name;
        write!(f, "{v}")?;
        Ok(())
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Item::fmt(&self, f)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Item::fmt(&self, f)
    }
}

struct ParserReader<T>
where
    T: Read,
{
    reader: BufReader<T>,
    buffer: Vec<char>,
    line: usize,
    column: usize,
    ctx: ParseContext,
}

struct ParseContext {
    items: Vec<Rc<Item>>,
}

trait ParserRead {
    fn read(&mut self) -> Result<char>;
    fn peek(&mut self) -> Result<char>;
    fn ctx(&mut self) -> &mut ParseContext;
    fn location(&self) -> (usize, usize);

    fn skip_space(&mut self) -> Result<()> {
        while self.peek()? == ' ' {
            _ = self.read()?;
        }
        Ok(())
    }
}

trait Parsed {
    fn first(c: char) -> bool;
    fn parse(reader: &mut impl ParserRead) -> Result<Self>
    where
        Self: Sized;
}

impl<T> ParserReader<T>
where
    T: Read,
{
    fn new(reader: T) -> ParserReader<T> {
        ParserReader {
            reader: BufReader::new(reader),
            buffer: Vec::<char>::new(),
            line: 0,
            column: 0,
            ctx: ParseContext { items: Vec::new() },
        }
    }
}

impl<T> ParserRead for ParserReader<T>
where
    T: Read,
{
    fn read(&mut self) -> Result<char> {
        if self.column < self.buffer.len() {
            let c = self.buffer[self.column];
            self.column += 1;
            return Ok(c);
        }

        let mut buf = String::new();
        let count = self.reader.read_line(&mut buf)?;
        if count == 0 {
            return Ok('\0');
        }
        self.line += 1;
        self.column = 1;
        self.buffer = buf.chars().collect();

        return Ok(self.buffer[0]);
    }

    fn peek(&mut self) -> Result<char> {
        if self.column < self.buffer.len() {
            return Ok(self.buffer[self.column]);
        }

        let mut buf = String::new();
        let count = self.reader.read_line(&mut buf)?;
        if count == 0 {
            return Ok('\0');
        }
        self.line += 1;
        self.column = 0;
        self.buffer = buf.chars().collect();

        return Ok(self.buffer[0]);
    }

    fn ctx(&mut self) -> &mut ParseContext {
        &mut self.ctx
    }

    fn location(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

fn err_unknown_char(reader: &mut impl ParserRead, target: String) -> Result<()> {
    let (line, column) = reader.location();
    let c = reader.peek()?;
    Err(Error::UnknownChar(target, c, line, column))
}

struct NewLine();

impl Parsed for NewLine {
    fn first(c: char) -> bool {
        c == '\n' || c == '\r'
    }

    fn parse(reader: &mut impl ParserRead) -> Result<Self> {
        let c1 = reader.peek()?;
        if c1 == '\n' {
            _ = reader.read()?;
            return Ok(NewLine());
        }

        if c1 == '\r' {
            if reader.peek()? == '\n' {
                _ = reader.read()?;
                return Ok(NewLine());
            }
        }

        todo!("unknown char")
    }
}

struct Speed(Fraction);

impl Parsed for Speed {
    fn first(c: char) -> bool {
        c.is_digit(10)
    }

    fn parse(reader: &mut impl ParserRead) -> Result<Self> {
        let mut v = String::new();
        loop {
            let c = reader.read()?;
            if c == 's' {
                break;
            }

            if c.is_digit(10) {
                v.push(c);
                continue;
            }
            todo!("unknown char");
        }

        let v = v.parse::<u32>()?;
        let time: Fraction = 60.into();
        Ok(Speed(time / v.into()))
    }
}

#[derive(Debug)]
pub struct Plan {
    pub formula: Rc<Formula>,
    pub speed: Fraction,
    pub group: Fraction,
    pub rate: Fraction,
}

impl Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Plan {
            formula,
            speed,
            group,
            rate,
        } = self;
        let rate = *rate * 100.into();
        write!(
            f,
            "group={group:2} speed={speed:6.2}/min rate={rate:6.2}% ==> "
        )?;
        Formula::fmt(formula, f, *speed / formula.target.amount().into())
    }
}
