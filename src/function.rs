pub(crate) fn process(
    input: crate::InputRecord,
) -> Result<crate::OutputRecord, Box<dyn std::error::Error>> {
    let output = crate::OutputRecord {
        id: input.id,
        price: input.price,
        tax: 0.095,
    };

    Ok(output)
}
