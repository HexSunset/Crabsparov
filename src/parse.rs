pub type Input<'a> = &'a str;
pub type Output<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;
