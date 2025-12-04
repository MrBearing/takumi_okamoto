use clap::{Parser, Subcommand};
use webbrowser;

const MY_GITHUB :&str = "https://github.com/MrBearing";
const MY_WEBSITE : &str = "http://mrbearing.github.io/";
const MY_QIITA : &str = "https://qiita.com/MrBearing";
const REQUEST_MY_CV : &str = "https://forms.gle/y2MDcCGq7TdSsq286";// Google form.


/// This program is the self introduction of Takumi Okamoto
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {
    #[clap(subcommand)]
    sub_command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// print all
    All,
    /// print my skills
    Skills {
        #[clap(subcommand)]
        category: Option<SkillCategory>,
    },
    /// open my web page in browser
    Website,
    /// open my Github page in browser
    Github,
    /// open my Qiita page in browser
    Qiita,
    /// open request form for my CV in browser
    CV,
}

#[derive(Subcommand, Clone, Copy)]
enum SkillCategory {
    /// Show only software skills
    Software,
    /// Show only mechanical skills
    Mechanical,
}

const MECHANICAL: &[&str] = &[
        "Robot system design",
        "Heat-resistant environment machine design",
        "Design of equipment for chemical-resistant (fluorine, etc.) environments, etc.",
];

const PROGRAMMING: &[&str] = &[
        "Rust",
        "ROS/ROS2",
        "python",
        "C/C++",
        "Elixir",
        "Kotlin",
        "Java SE 1.6~1.8 frameWork: SpringFramework 4",
];

fn describe_all(){
        println!("Name : Takumi Okamoto");
        println!("~~ Mechanical Engineer. Sometimes wrote software ~~");
        describe_skill(None);
        println!("Github address : {}",MY_GITHUB);
        println!("web : {}",MY_WEBSITE);
        println!("Qiita :{}",MY_QIITA);
        println!("CV :{}",REQUEST_MY_CV);
}

/// A subcommand for printing my skills
fn describe_skill(category: Option<SkillCategory>){
        println!("**skills***\n");
        match category {
                Some(SkillCategory::Mechanical) => print_skill_section("Mechanical design", MECHANICAL),
                Some(SkillCategory::Software) => print_skill_section("Programings", PROGRAMMING),
                None => {
                        print_skill_section("Mechanical design", MECHANICAL);
                        println!();
                        print_skill_section("Programings", PROGRAMMING);
                }
        }
        println!();
}

fn print_skill_section(title: &str, items: &[&str]) {
        println!("{title}");
        for item in items {
                println!("{0: <5} {1: <5}", "", item);
        }
}

fn open_link(label: &str, url: &str) {
        if !webbrowser::Browser::is_available() {
                println!("{label} : {url}");
                return;
        }

        if let Err(err) = webbrowser::open(url) {
                eprintln!("Failed to open {label} ({url}): {err}");
        }
}


fn main() {
    let options = Cli::parse();
    match options.sub_command {
        Commands::All => describe_all(),
        Commands::Skills { category } => describe_skill(category),
        Commands::Github => open_link("Github", MY_GITHUB),
        Commands::Website => open_link("Website", MY_WEBSITE),
        Commands::Qiita => open_link("Qiita", MY_QIITA),
        Commands::CV=> open_link("CV", REQUEST_MY_CV),
    }
}
