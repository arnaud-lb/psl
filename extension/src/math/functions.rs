use phper::values::ZVal;

pub fn to_base(arguments: &mut [ZVal]) -> phper::Result<String> {
    let mut number = arguments[0].expect_long()?;
    let base = arguments[1].expect_long()?;

    let mut result = vec![];

    loop {
        let n = number % base;
        number /= base;

        // panics if `base` is < 2 or > 32, PSL php implementation
        // also fails, and considers passing invalid value
        // an undefined behavior, so panicing is not really an
        // issue, respect the function signature.
        result.push(char::from_digit(n.try_into().unwrap(), base.try_into().unwrap()).unwrap());
        if number == 0 {
            break;
        }
    }

    Ok(result.into_iter().rev().collect::<String>())
}

pub fn from_base(arguments: &mut [ZVal]) -> phper::Result<i64> {
    let number = arguments[0].as_z_str().unwrap().to_str()?;
    let base = arguments[1].expect_long()?;

    Ok(i64::from_str_radix(number, base.try_into().unwrap()).unwrap())
}
