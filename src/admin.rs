#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
mod database;
pub mod schema;
mod utils;

use crate::utils::get_password_or_prompt;


use clap::{App, Arg, SubCommand, ArgMatches};

fn list_users () {
    let connection = database::establish_connection();
    let users = database::list_users(&connection);
    for user in users {
        println!("{}/{}", user.username, user.pw_hash)
    }
}

fn add_project (matches: &ArgMatches) {
    let connection = database::establish_connection();
    let project_name = matches.value_of("name").unwrap();
    let project = database::create_project(&connection, &project_name);
    println!("{}", serde_json::to_string_pretty(&project).unwrap());
}

fn list_projects() {
    let connection = database::establish_connection();
    let projects = database::get_projects(&connection);
    println!("{}", serde_json::to_string_pretty(&projects).unwrap());
}

fn show_project(matches: &ArgMatches) {
    let connection = database::establish_connection();
    let name = matches.value_of("name").unwrap();
    let project = database::get_project_by_name(&connection, &name);
    let api_project = project.get_api_object(Some(&connection));
    println!("{}", serde_json::to_string_pretty(&api_project).unwrap());
}

fn delete_project(matches :&ArgMatches) {
    let connection = crate::database::establish_connection();
    let target_project = matches.value_of("name").unwrap();
    let num_deleted = database::delete_project(&connection, &target_project);
    println!("Deleted {} projects", num_deleted);
}

fn add_user (matches: &ArgMatches) {
    let connection = database::establish_connection();
    let username = matches.value_of("username").unwrap();
    println!("{}", username);
    let password = get_password_or_prompt(&matches);
    let user = database::create_user(&connection, &username, &password);
    println!("Saved user {}", user.username);
}

fn change_user_password(matches: &ArgMatches) {
    let connection = crate::database::establish_connection();
    let username = matches.value_of("username").unwrap();
    let password = get_password_or_prompt(&matches);
    database::change_user_password(&connection, &username, &password);
    println!("Password changed")
}

fn verify_password(matches: &ArgMatches) {
    let connection = crate::database::establish_connection();
    let username = matches.value_of("username").unwrap();
    let password = get_password_or_prompt(&matches);
    let result = database::check_password(&connection, &username, &password);
    match result {
        true => println!("Correct password"),
        false => println!("Incorrect password")
    }
}

fn delete_user (matches: &ArgMatches) {
    let connection = crate::database::establish_connection();
    let target_username = matches.value_of("username").unwrap();
    let num_deleted = database::delete_user(&connection, &target_username);
    println!("Deleted {} users", num_deleted);
}

fn add_user_to_project(matches: &ArgMatches) {
    let connection = crate::database::establish_connection();
    let target_username = matches.value_of("username").unwrap();
    let target_project = matches.value_of("project").unwrap();
    let view_role = matches.is_present("view_role");
    let modify_role = matches.is_present("modify_role");
    let admin_role = matches.is_present("admin_role");
    let permissions = database::ProjectUserPermissions {
        view_role: view_role,
        modify_role: modify_role,
        admin_role: admin_role
    };
    let user = crate::database::get_user_by_name(&connection, &target_username);
    let project = crate::database::get_project_by_name(&connection, &target_project);
    let project_user = crate::database::add_user_to_project(&connection, user, project, permissions);
    println!("Added project user {}", project_user.id);
}

fn main () {
    // Command line options and paramaters (uses clap module)
    let matches = App::new("metalmon-admin")
        .version("0.1.0")
        .author("Grayson Head <grayson@graysonhead.net>")
        .about("Admin utility for administering metalmon")
        .subcommand(
            SubCommand::with_name("add_user")
                .about("Adds users to metalmon")
                .arg(
                    Arg::with_name("username")
                        .index(1)
                        .required(true)
                        .help("Name of the user to add")
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .required(false)
                        .help("Sets the password of the user to add. If not present, you will be prompted for a password")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("delete_user")
                .about("Removes users from metalmon")
                .arg(
                    Arg::with_name("username")
                        .index(1)
                        .required(true)
                        .help("Name of the user to delete")
                )
        )
        .subcommand(
            SubCommand::with_name("list_users")
                .about("Lists users")
        )
        .subcommand(
            SubCommand::with_name("change_user_password")
                .about("Change user password")
                .arg(
                    Arg::with_name("username")
                        .index(1)
                        .required(true)
                        .help("Name of the user who's password to change")
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .required(false)
                        .help("Sets the new password. If not specified, you will be prompted")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("verify_password")
                .about("Check validity of users password")
                .arg(
                    Arg::with_name("username")
                        .index(1)
                        .required(true)
                        .help("Name of the user who's password to check")
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .required(false)
                        .help("Password to verify. If not specified, you will be prompted")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("add_project")
                .about("Create new project")
                .arg(
                    Arg::with_name("name")
                    .index(1)
                    .required(true)
                    .help("Name of the new project")
                )
        )
        .subcommand(
            SubCommand::with_name("delete_project")
                .about("Delete a project")
                .arg(
                    Arg::with_name("name")
                    .index(1)
                    .required(true)
                    .help("Name of the project to delete")
                )
        )
        .subcommand(
            SubCommand::with_name("list_projects")
                .about("List all projects")
        )
        .subcommand(
            SubCommand::with_name("add_user_to_project")
                .about("Add a user to a project")
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .required(true)
                        .takes_value(true)
                        .help("User to add to the project")
                )
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .required(true)
                        .takes_value(true)
                        .help("Project to add user to")
                )
                .arg(
                    Arg::with_name("view_role")
                        .long("view_role")
                )
                .arg(
                    Arg::with_name("modify_role")
                        .long("modify_role")
                )
                .arg(
                    Arg::with_name("admin_role")
                        .long("admin_role")
                )
        )
        .subcommand(
            SubCommand::with_name("get_project")
                .about("Show project")
                .arg(
                    Arg::with_name("name")
                    .index(1)
                    .required(true)
                )
        )
        .get_matches();

    
    if let Some(matches) = matches.subcommand_matches("add_user") {
        add_user(&matches);
    }
    if let Some(matches) = matches.subcommand_matches("delete_user") {
        delete_user(&matches);
    }
    if let Some(_matches) = matches.subcommand_matches("list_users") {
        list_users();
    }
    if let Some(matches) = matches.subcommand_matches("change_user_password") {
        change_user_password(&matches);
    }
    if let Some(matches) = matches.subcommand_matches("verify_password"){
        verify_password(&matches);
    }
    if let Some(matches) = matches.subcommand_matches("add_project") {
        add_project(&matches);
    }
    if let Some(_matches) = matches.subcommand_matches("list_projects") {
        list_projects();
    }
    if let Some(matches) = matches.subcommand_matches("delete_project") {
        delete_project(&matches);
    }
    if let Some(matches) = matches.subcommand_matches("add_user_to_project") {
        add_user_to_project(&matches);
    }
    if let Some(matches) = matches.subcommand_matches("get_project") {
        show_project(&matches);
    }
    
}