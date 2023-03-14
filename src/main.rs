use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_system(print_names)
        .add_system(people_with_jobs)
        .add_system(people_ready_for_hire)
        .add_system(person_does_job);

    }
}


pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Sanket".to_owned(),
        },
        Employed { job: Job::Doctor },
    ));
    //has no job
    commands.spawn(Person {
        name: "Bob".to_owned(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_owned(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "Ellen".to_owned(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "Alex".to_owned(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for job", person.name );
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("{} is a {:?}", person.name, employed.job);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
