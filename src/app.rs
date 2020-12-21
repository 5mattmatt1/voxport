
pub fn new_app<'help>() -> clap::App<'help>
{
    let app = clap::App::new("Voxport")
        .arg(
            clap::Arg::new("stl")
            .about("Exports in the STL (STereoLithography) format. Good for 3D Printing")
            .short('s')
            .long("stl")
            .required(true)
            .conflicts_with("dae")
        )
        .arg(
            clap::Arg::new("dae")
            .about("Exports in the Collada DAE format. Good for importing")
            .short('d')
            .long("dae")
            .required(true)
            .conflicts_with("stl")
        )
        .arg(
            clap::Arg::new("input")
            .about("Input MagicaVoxel file to convert")
            .short('i')
            .long("input")
            .takes_value(true)
        )
        .arg(
            clap::Arg::new("output")
            .about("Output file of specified export format")
            .short('o')
            .long("output")
            .takes_value(true)
        )
    ;
    return app;
}