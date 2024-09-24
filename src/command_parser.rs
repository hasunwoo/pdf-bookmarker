#[derive(clap::Parser)]
#[command(
    name = "pdfmark",
    about = "A CLI tool for applying bookmark to pdf file."
)]
pub struct Command {
    #[command(subcommand)]
    pub command: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    #[command(about = "Applies toc file to pdf's outline")]
    Apply {
        #[arg(
            short = 'i',
            long = "input",
            value_name = "FILE",
            help = "Path to input PDF file"
        )]
        input_file: String,
        #[arg(
            short = 't',
            long = "toc",
            value_name = "FILE",
            help = "Path to input toc file"
        )]
        toc_file: String,
        #[arg(
            long = "offset",
            help = "Page offset. If set, It will be added to pages number in toc file."
        )]
        offset: Option<i32>,
        #[arg(
            short = 'p',
            long = "password",
            value_name = "FILE",
            help = "Password of PDF file if pdf is encrypted"
        )]
        password: Option<String>,
        #[arg(short = 'f', long = "force", action = clap::ArgAction::SetTrue, help = "If set, It will overwrite Outlines in PDF file.")]
        force: bool,
        #[arg(short = 'w', long = "write", action = clap::ArgAction::SetTrue, help = "If set, It will overwrite destination file.")]
        write: bool,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Path to output PDF file"
        )]
        output_file: String,
    },
    #[command(about = "Clears bookmarks(outlines) from PDF file")]
    Clear {
        #[arg(
            short = 'i',
            long = "input",
            value_name = "FILE",
            help = "Path to input PDF file"
        )]
        input_file: String,
        #[arg(
            short = 'p',
            long = "password",
            value_name = "FILE",
            help = "Password of PDF file if pdf is encrypted"
        )]
        password: Option<String>,
        #[arg(short = 'w', long = "write", action = clap::ArgAction::SetTrue, help = "If set, It will overwrite destination file.")]
        write: bool,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Path to output PDF file"
        )]
        output_file: String,
    },
    #[command(about = "Extracts bookmarks from existing PDF file")]
    Extract {
        #[arg(
            short = 'i',
            long = "input",
            value_name = "FILE",
            help = "Path to input PDF file"
        )]
        input_file: String,
        #[arg(
            short = 'p',
            long = "password",
            value_name = "FILE",
            help = "Password of PDF file if pdf is encrypted"
        )]
        password: Option<String>,
        #[arg(long = "print", action = clap::ArgAction::SetTrue, help = "If set, It will print toc format to console instead of writing to file.")]
        print: bool,
        #[arg(short = 'w', long = "write", action = clap::ArgAction::SetTrue, help = "If set, It will overwrite destination file.")]
        write: bool,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Path to output toc file"
        )]
        output_file: Option<String>,
    },
    #[command(about = "Validates toc file")]
    Validate {
        #[arg(
            short = 't',
            long = "toc",
            value_name = "FILE",
            help = "Path to input toc file"
        )]
        toc_file: String,
    },
}
