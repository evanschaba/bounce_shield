use bevy::prelude::*;

fn main() {
    let x = 2_000_000;
    let y = 40_000;
    println!("{} / {} = {}", x, y, x / y)
    // App ::new().add_plugins(MyPlug).run();
}

// calculate:  = ?

// improve this please
pub struct MyPlug;

impl Plugin for MyPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_employed_humans)
            .add_systems(Update, print_un_employed_humans)
            .add_systems(Update, print_in_active_names)
            .add_systems(Update, print_active_names);
    }
}

#[derive(Debug, Clone)]
pub enum Profession {
    None,
    Mechanic,
    Engineer,
    Researcher,
    Teacher,
    Musician,
    Doctor,
    Driver,
    Accountant,
    Manager,
    Investor,
}

#[derive(Component, Debug)]
pub struct Employment {
    pub profession: Profession,
}

#[derive(Component)]
pub struct Human {
    pub name: String,
    pub active: bool,
}
pub fn setup(mut cmds: Commands) {
    let names_and_activeness = vec![
        ("Alice", true),
        ("Jaqueline", true),
        ("Janice", true),
        ("Jess", false),
        ("Suheila", true),
        ("Mariam", true),
        ("Rose", false),
        ("Alexandra", false),
        ("Emily", false),
        ("Anne", true),
        ("Sharon", true),
        ("Esther", false),
        ("Lucia", true),
    ];

    let professions = vec![
        Profession::None,
        Profession::Mechanic,
        Profession::Engineer,
        Profession::Researcher,
        Profession::Teacher,
        Profession::Musician,
        Profession::Doctor,
        Profession::Driver,
        Profession::Accountant,
        Profession::Manager,
        Profession::Investor,
        Profession::None,
    ];

    for ((name, active), profession) in names_and_activeness
        .into_iter()
        .zip(professions.iter().cycle())
    {
        cmds.spawn((
            Human {
                name: name.to_string(),
                active,
            },
            Employment {
                profession: profession.clone(),
            },
        ));
    }
}

pub fn print_active_names(q: Query<(&Human, &Employment)>) {
    q.iter()
        .filter_map(|(person, employment)| {
            if person.active {
                Some((&person.name, employment))
            } else {
                None
            }
        })
        .for_each(|(name, employment)| {
            println!("name: {}, profession: {:?}", name, employment.profession)
        });
    println!("\n")
}

pub fn print_in_active_names(q: Query<(&Human, &Employment)>) {
    q.iter()
        .filter_map(|(person, employment)| {
            if !person.active {
                Some((&person.name, employment))
            } else {
                None
            }
        })
        .for_each(|(name, employment)| {
            println!("name: {}, profession: {:?}", name, employment.profession)
        });
    println!("\n")
}

pub fn print_employed_humans(q: Query<&Human, With<Employment>>) {
    q.iter().for_each(|h| println!("name: {}", h.name));
}

pub fn print_un_employed_humans(q: Query<&Human, Without<Employment>>) {
    q.iter().for_each(|h| println!("name: {}", h.name));
}
