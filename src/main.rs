use clap::Clap;
use webbrowser;

const MY_GITHUB :&str = "https://github.com/MrBearing";
const MY_WEBSITE : &str = "http://mrbearing.github.io/";
const MY_QIITA : &str = "https://qiita.com/MrBearing";


/// This program is the self introduction of Takumi Okamoto's
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Takumi Okamoto <takumi1988okamoto@gmail.com>")]
struct Options {
    #[clap(subcommand)]
    sub_comand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    All(All),
    Skills(Skills),
    Website(Website),
    Github(Github),
    Qiita(Qiita),
}

/// A subcommand for opennig my website
#[derive(Clap)]
struct Website {}

/// A subcommand for opennig my Github
#[derive(Clap)]
struct Github {}
/// A subcommand for opennig my Qiita
#[derive(Clap)]
struct Qiita {}

/// A subcommand for printing all my profile
#[derive(Clap)]
struct All { }

fn describe_all(){
        println!("Name : Takumi Okamoto");
        println!("~~ Mechanical Engineer. sometimes wrote software ~~");
        describe_skill();
        println!("Github address : {}",MY_GITHUB);
        println!("web : {}",MY_WEBSITE);
        println!("Qiita :{}",MY_QIITA)
}



/// A subcommand for printing my skills
#[derive(Clap)]
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
        println!("{0: <5} {1: <5}","","ROS/ ROS2");
        println!("{0: <5} {1: <5}","","C/C++");
        println!("{0: <5} {1: <5}","","Java SE 1.6~1.8 frameWork: SpringFramework 4");
        println!("{0: <5} {1: <5}","","python");
        println!("{0: <5} {1: <5}","","Kotlin");
        println!("");
}


fn main() {
    let options: Options = Options::parse();
    match options.sub_comand {
        SubCommand::All(_) => describe_all(),
        SubCommand::Skills(_) => describe_skill(),
        SubCommand::Github(_) => {webbrowser::open(MY_GITHUB).unwrap(); ()},
        SubCommand::Website(_) => {webbrowser::open(MY_GITHUB).unwrap(); ()},
        SubCommand::Qiita(_) => {webbrowser::open(MY_GITHUB).unwrap(); ()},
    }
}