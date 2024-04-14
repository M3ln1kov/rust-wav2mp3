pub struct WavFile {
    pub header: WavHeader,
    pub data: Vec<u8>,
}

pub struct WavHeader {
    pub chunk_id: [u8; 4],
    pub chunk_size: u32,
    pub format: [u8; 4],
    pub subchunk1_id: [u8; 4],
    pub subchunk1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub subchunk2_id: [u8; 4],
    pub subchunk2_size: u32,
}

pub struct FmtData
{
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,

}

impl WavFile {
    pub fn new(header: WavHeader, data: Vec<u8>) -> WavFile {
        WavFile { header, data }
    }
}

pub fn read_wav_header()
{
    let header = WavHeader {
        chunk_id: [b'R', b'I', b'F', b'F'],
        chunk_size: 36,
        format: [b'W', b'A', b'V', b'E'],
        subchunk1_id: [b'f', b'm', b't', b' '],
        subchunk1_size: 16,
        audio_format: 1,
        num_channels: 1,
        sample_rate: 44100,
        byte_rate: 88200,
        block_align: 2,
        bits_per_sample: 16,
        subchunk2_id: [b'd', b'a', b't', b'a'],
        subchunk2_size: 88200,
    };

    let data = vec![0; 88200];
    let wav = WavFile::new(header, data);
    println!("Wav file: {:?}", wav);
}

pub fn read_wav_file(filename String, )
{
    let mut file = File::open(filename).unwrap();
    let mut header = WavHeader {
        chunk_id: [0; 4],
        chunk_size: 0,
        format: [0; 4],
        subchunk1_id: [0; 4],
        subchunk1_size: 0,
        audio_format: 0,
        num_channels: 0,
        sample_rate: 0,
        byte_rate: 0,
        block_align: 0,
        bits_per_sample: 0,
        subchunk2_id: [0; 4],
        subchunk2_size: 0,
    };

    file.read_exact(&mut header.chunk_id).unwrap();
    file.read_exact(&mut header.chunk_size).unwrap();
    file.read_exact(&mut header.format).unwrap();
    file.read_exact(&mut header.subchunk1_id).unwrap();
    file.read_exact(&mut header.subchunk1_size).unwrap();
    file.read_exact(&mut header.audio_format).unwrap();
    file.read_exact(&mut header.num_channels).unwrap();
    file.read_exact(&mut header.sample_rate).unwrap();
    file.read_exact(&mut header.byte_rate).unwrap();
    file.read_exact(&mut header.block_align).unwrap();
    file.read_exact(&mut header.bits_per_sample).unwrap();
    file.read_exact(&mut header.subchunk2_id).unwrap();
    file.read_exact(&mut header.subchunk2_size).unwrap();

    let mut data = vec![0; header.subchunk2_size as usize];
    file.read_exact(&mut data).unwrap();

    let wav = WavFile::new(header, data);
    println!("Wav file: {:?}", wav);
}

// write a function which checks the header of a wav file
pub fn check_wav_header(header: WavHeader) -> bool
{
    if header.chunk_id != [b'R', b'I', b'F', b'F'] {
        return false;
    }
    if header.format != [b'W', b'A', b'V', b'E'] {
        return false;
    }
    if header.subchunk1_id != [b'f', b'm', b't', b' '] {
        return false;
    }
    if header.audio_format != 1 {
        return false;
    }
    if header.num_channels != 1 {
        return false;
    }
    if header.sample_rate != 44100 {
        return false;
    }
    if header.byte_rate != 88200 {
        return false;
    }
    if header.block_align != 2 {
        return false;
    }
    if header.bits_per_sample != 16 {
        return false;
    }
    if header.subchunk2_id != [b'd', b'a', b't', b'a'] {
        return false;
    }
    true
}