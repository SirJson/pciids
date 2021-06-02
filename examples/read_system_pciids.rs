use pciids::PciIdData;

use anyhow::Result;

#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(long = "pci-ids-file")]
    #[cfg_attr(
        target_os = "linux",
        structopt(default_value = "/usr/share/misc/pci.ids")
    )]
    #[cfg_attr(target_os = "redox", structopt(default_value = "/share/misc/pci.ids"))]
    pci_ids_file: String,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let mut pci_id_data = PciIdData::new();
    let pci_id_file_contents =
        std::fs::read_to_string(args.pci_ids_file).expect("cannot read file");
    pci_id_data.add_pci_ids_data(&mut pci_id_file_contents.as_bytes())?;

    println!("{:#?}", pci_id_data);
    Ok(())
}
