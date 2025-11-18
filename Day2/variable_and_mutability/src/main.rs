//type aliases

type Meters = i32 ;

fn main()
{
    //variable
    let apple = 50;
    let oranges = 32;
    let fruits = apple+oranges;
    println!("Last year my garden had {2} apples, {1} oranges and total {0} fruits",apple,oranges,fruits);

    //mutability
    let mut gym_reps = 12;
    println!("Gym reps are : {}",gym_reps);
    gym_reps = 16;
    println!("Gym reps are : {}",gym_reps);

    //constants

    const PI : f64 = 3.14;
    const INCOME : i32 = 3000;
    println!("The value of pi is {},and my monthly income is {}",PI,INCOME);

    let mile_race_length : Meters = 1600;
    let two_mile_race : Meters = 3200;

    println!("the length of mile race is {} and two mile race is {}",mile_race_length,two_mile_race);
}