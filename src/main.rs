use bevy::prelude::*;

struct Person;

struct Name(String);

fn add_people(mut commands: Commands){
    commands
        .spawn((Person, Name("LuFeMas".to_string())))
        .spawn((Person, Name("LuMimikyu".to_string())))
        .spawn((Person, Name("SueReg".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::build()
    .add_startup_system(add_people.system())
    .add_system(greet_people.system())
    .add_system(hello_world.system())
    .run();
}