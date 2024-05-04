
/**
 * Structs
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main(){
    let user1: User = build_user(String::from("First username"), String::from("First email"));

    /* Dans ce cas, .username n'est plus accessible d'apres user1 */
    let user2: User = User {
        email: String::from("Second email"),
        ..user1
    };

    /* Si on essaye de voir ce qui se pass a l'interieur de user 1, on obtient: */
    /* println!("user1: {}", user1.username); */

    /* Pourtant on peut toujour voir son nom d'apres user 2: */
    println!("user2.active: {}, \nuser2.signin: {}", user2.active, user2.sign_in_count);
    println!("user2.username: {} and \nuser2.email: {}", user2.username, user2.email);

    let black : Color = Color(0, 1, 2);
    let center: Point = Point(3, 4, 5);

    println!("in the middle of black: {}", black.1);
    println!("in the end of center : {}", center.2);

    

}


fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
