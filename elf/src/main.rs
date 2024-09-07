use elf::endian::AnyEndian;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;
use elf::segment::ProgramHeader;
use elf::ElfBytes;
use memmap2::Mmap;
use object::{Object, ObjectSection};
use std::fs::File;
use std::path::PathBuf;

fn main() {
    // i need aarch cus im mac rn
    let path: PathBuf = std::path::PathBuf::from("./hello_world.elf");
    // let file_data = std::fs::read(path).expect("Could not read file.");
    // let elf_file = object::File::parse(&*file_data).expect("Failed to parse ELF file");

    let file = File::open(&path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let elf_file = object::File::parse(&*mmap).expect("Failed to parse ELF file");

    println!("Entry point: 0x{:x}", elf_file.entry());

    // I can get elf sections like this!
    // for section in elf_file.sections() {
    //     println!("{}", section.name().unwrap());
    // }

    // let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(&mmap).expect("Open test1");
    // Analyze program headers (segments)
    for ph in file.segments().unwrap() {
        println!("Segment: {:?}", ph.p_type);
        println!("  Virtual Address: 0x{:x}", ph.p_vaddr);
        println!("  File size: {}", ph.p_filesz);
        println!("  Memory size: {}", ph.p_memsz);
    }

    let load_segments: Vec<ProgramHeader> = file
        .segments()
        .unwrap()
        .into_iter()
        .filter(|ph| ph.p_type == elf::abi::PT_LOAD)
        .collect();

    println!("Loadable segments: {}", load_segments.len());

    // for section in file.section_headers() {
    //     println!("Section name: {:?}", section.);
    // }

    // Get the ELF file's build-id
    // let abi_shdr: SectionHeader = file
    //     .section_header_by_name(".note.gnu.build-id")
    //     .expect("section table should be parseable")
    //     .expect("file should have a .note.ABI-tag section");

    // let notes: Vec<Note> = file
    //     .section_data_as_notes(&abi_shdr)
    //     .expect("Should be able to get note section data")
    //     .collect();
    // assert_eq!(
    //     notes[0],
    //     Note::GnuBuildId(NoteGnuBuildId(&[
    //         254, 112, 219, 225, 142, 178, 110, 61, 42, 8, 93, 220, 128, 190, 201, 157, 107, 145,
    //         94, 54
    //     ]))
    // );

    // // Find lazy-parsing types for the common ELF sections (we want .dynsym, .dynstr, .hash)
    // let common = file.find_common_data().expect("shdrs should parse");
    // let (dynsyms, strtab) = (common.dynsyms.unwrap(), common.dynsyms_strs.unwrap());
    // println!("dynsyms:{:?}", dynsyms);
    // println!("strtab:{:?}", strtab);
    // let hash_table = common.sysv_hash.unwrap();

    // // Use the hash table to find a given symbol in it.
    // let name = b"memset";
    // let (sym_idx, sym) = hash_table
    //     .find(name, &dynsyms, &strtab)
    //     .expect("hash table and symbols should parse")
    //     .unwrap();

    // // Verify that we got the same symbol from the hash table we expected
    // assert_eq!(sym_idx, 5);
    // assert_eq!(strtab.get(sym.st_name as usize).unwrap(), "memset");
    // assert_eq!(sym, dynsyms.get(sym_idx).unwrap());
}
