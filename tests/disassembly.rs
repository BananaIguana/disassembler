use {
    disasm_frontend::{
        arch::Architecture,
        decoder::{decoder, Decoder},
        mode::Mode,
    },
    std::io::Read,
};

pub fn load_sample(index: usize) -> Vec<u8>
{
    let path = format!("res/sample-{:02}.bin", index);
    let mut file = std::fs::File::open(&path).unwrap_or_else(|_| panic!("Failed to open '{}'", &path));
    let mut data = Vec::<u8>::new();

    file.read_to_end(&mut data)
        .unwrap_or_else(|_| panic!("Failed to read '{}'", &path));

    data
}

#[test]
pub fn disassembly_test()
{
    let data = load_sample(1);

    assert_eq!(data.len(), 320);

    let decoder = decoder(Architecture::Arm64);

    let result = decoder.decode(&data, Mode::Strict);
}
