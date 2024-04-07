use clap::{Parser, Subcommand};
use webbrowser;

const MY_GITHUB :&str = "https://github.com/MrBearing";
const MY_WEBSITE : &str = "http://mrbearing.github.io/";
const MY_QIITA : &str = "https://qiita.com/MrBearing";
const REQUEST_MY_CV : &str = "https://forms.gle/y2MDcCGq7TdSsq286";// Google form.



/// This program is the self introduction of Takumi Okamoto's
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Takumi Okamoto <takumi1988okamoto@gmail.com>")]
struct Cli {
    #[clap(subcommand)]
    sub_comand: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// print all
    All,
    /// print my skills
    Skills,
    /// open my web page in browser
    Website,
    /// open my Github page in browser
    Github,
    /// open my Qiita page in browser
    Qiita,
    /// open request form for my CV in browser
    CV,
}


fn describe_all(){
        println!("Name : Takumi Okamoto");
        println!("~~ Mechanical Engineer. Sometimes wrote software ~~");
        describe_skill();
        println!("Github address : {}",MY_GITHUB);
        println!("web : {}",MY_WEBSITE);
        println!("Qiita :{}",MY_QIITA);
        println!("CV :{}",REQUEST_MY_CV);
}

/// A subcommand for printing my skills
#[derive(Parser)]
struct Skills {}

fn describe_skill(){
        // 配列でやりたい。。。
        println!("**skills***");
        println!("");
        println!("Mechanical design");
        println!("{0: <5} {1: <5}","","Robot system design");
        println!("{0: <5} {1: <5}","","Heat-resistant environment machine design");
        println!("{0: <5} {1: <5}","","Design of equipment for chemical-resistant (fluorine, etc.) environments, etc.");
        println!("");
        println!("Programings");
        println!("{0: <5} {1: <5}","","Rust");
        println!("{0: <5} {1: <5}","","Elixir");
        println!("{0: <5} {1: <5}","","ROS/ROS2");
        println!("{0: <5} {1: <5}","","C/C++");
        println!("{0: <5} {1: <5}","","python");
        println!("{0: <5} {1: <5}","","Kotlin");
        println!("{0: <5} {1: <5}","","Java SE 1.6~1.8 frameWork: SpringFramework 4");
        println!("");
}


fn main() {
    let options = Cli::parse();
    match options.sub_comand {
        Commands::All => describe_all(),
        Commands::Skills => describe_skill(),
        Commands::Github => {webbrowser::open(MY_GITHUB).unwrap(); ()},
        Commands::Website => {webbrowser::open(MY_WEBSITE).unwrap(); ()},
        Commands::Qiita => {webbrowser::open(MY_QIITA).unwrap(); ()},
        Commands::CV=> {webbrowser::open(REQUEST_MY_CV).unwrap();()},
    }
}