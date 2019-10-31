use std::io::Write;

use crate::Result;

const ATTRIBUTES: [(anes::Attribute, anes::Attribute); 8] = [
    (anes::Attribute::Bold, anes::Attribute::Normal),
    (anes::Attribute::Faint, anes::Attribute::Normal),
    (anes::Attribute::Italic, anes::Attribute::ItalicOff),
    (anes::Attribute::Underline, anes::Attribute::UnderlineOff),
    (anes::Attribute::Reverse, anes::Attribute::ReverseOff),
    (anes::Attribute::Conceal, anes::Attribute::ConcealOff),
    (anes::Attribute::Crossed, anes::Attribute::CrossedOff),
    (anes::Attribute::Blink, anes::Attribute::BlinkOff),
];

fn test_set_display_attributes<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(w, "Display attributes", anes::MoveCursorToNextLine(2),)?;

    for (on, off) in &ATTRIBUTES {
        queue!(
            w,
            anes::SetAttribute(*on),
            format!("{:>width$} ", format!("{:?}", on), width = 35),
            anes::SetAttribute(*off),
            format!("{:>width$}", format!("{:?}", off), width = 35),
            anes::ResetAttributes,
            anes::MoveCursorToNextLine(1),
        )?;
    }

    w.flush()?;

    Ok(())
}

#[allow(clippy::cognitive_complexity)]
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(w, test_set_display_attributes,);
    Ok(())
}
