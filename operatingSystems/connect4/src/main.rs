#[macro_use] extern crate text_io;
extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::f32::INFINITY;

static COMPPIECE: char = '0';
static PLAYERPIECE: char = 'x';

fn main()
{
    let mut pieces: [[char; 6]; 7] = [[' '; 6]; 7];

    println!("Hi there");
    pvc_game(&mut pieces);
    //testing
}

fn pvc_game(mut pieces: &mut[[char; 6]; 7])
{
    let mut m: usize;
    let mut rng = rand::thread_rng();
    let gen = Range::new(1, 7);

    print_board(&pieces);

    loop
    {
        print!("Make your move.\n");
        m = read!();
        make_move('x', m, &mut pieces);
        print_board(&pieces);
        //m = gen.ind_sample(&mut rng);
        //make_move('o', m, &mut pieces);
        m = read!();
        make_move('o', m, &mut pieces);
        print_board(&pieces);
        println!("Evaluation: {}", evaluate(&pieces));
    }
}

fn print_board(pieces: &[[char; 6]; 7])
{
    /*  Rows
    |6|6|6|6|6|6|
    |5|5|5|5|5|5|
    |4|4|4|4|4|4|
    |3|3|3|3|3|3|
    |2|2|2|2|2|2|
    |1|1|1|1|1|1|
    |0|0|0|0|0|0|
     1 2 3 4 5 6     */

    /* Columns
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
    |0|1|2|3|4|5|
     1 2 3 4 5 6    */

    for row in (0..7).rev()
    {
        for column in 0..6
        {
            print!("|{}", pieces[row][column]);
        }
        print!("|\n");
    }
    print!(" 1 2 3 4 5 6\n\n");
}

fn make_move(piece: char, spot: usize, pieces: &mut [[char; 6]; 7])
{
    print!("spot: {}\n", spot);
    //let i: usize;
    for i in 0..7
    {
        println!("pieces[i][spot-1]: {}", pieces[i][spot-1]);
        if pieces[i][spot-1] == ' '
        {
            pieces[i][spot-1] = piece;
            break;
        }
    }
}

#[allow(dead_code)]
fn ai_move(pieces: [[char; 6]; 7])->i32
{
    3
}

#[allow(dead_code)]
fn max(depth: i32, pieces: [[char; 6]; 7])->i32
{
    let best: i32 = INFINITY as i32;

    if depth <= 0 
        {return evaluate(&pieces);}
    
    generate_legal_moves(pieces);


    best
}

#[allow(dead_code)]
fn minmax(depth: i32, pieces: [[char; 6]; 7])->i32
{
    3
}

fn evaluatev2(pieces: &[[char; 6]; 7])->i32
{
    let mut score = 0;
    let mut in_a_row = 0;
    let mut empty_sides = 0;

    for row in 0..7
    {
        for column in 0..6
        {
            if pieces[row][column] == 'o'
            {
                //3-4:30-6- 7:30-9-10:30-12

                //3:00
                for column_spot in column..6
                {
                    
                }
            }
        }
    }

    let eval1 = |piece|
    {
    //in_a_row, empty_sides, score
        match piece
        {
            //computer
            'o' => in_a_row += 1,

            //player
            'x' => 
            {
                if in_a_row.is_positive() && empty_sides.is_positive()
                { 
                    score += in_a_row * empty_sides;
                    in_a_row = 0;
                }
                empty_sides = 0;
            },        

            //empty
            ' ' => 
            {
                //TODO: Check if this is a gap. Also make scoring exponential.

                if in_a_row.is_positive()
                {
                    if column == 5 || pieces[row][column+1] != 'o'
                    {
                        empty_sides += 1;
                        score += in_a_row * empty_sides;
                        in_a_row = 0;
                    }
                }
                else{
                    empty_sides = 1; 
                }
                
            },

            _ => println!("wtf")
        }
    }; //end eval1
    3
}





fn evaluate(pieces: &[[char; 6]; 7])->i32
{
    //For Friendly: +128 for 4 in a row. 
    //+8 for 3 in a row with open. 
    //+2 for 2 in a row with open.

    //pieces is an array of 7 rows of 6   

    let mut score: i32 = 0;
    let mut in_a_row: i32 = 0;
    let mut empty_sides: i32 = 0;

    for row in 0..7
    {
        for column in 0..6
        {
            match pieces[row][column]
            {
                //computer
                'o' => in_a_row+=1,

                //player
                'x' => 
                {
                    if in_a_row.is_positive() && empty_sides.is_positive()
                    { 
                        score += in_a_row * empty_sides;
                        in_a_row = 0;
                    }
                    empty_sides = 0;
                },
                
                //empty
                ' ' => 
                {
                    //TODO: Check if this is a gap. Also make scoring exponential.

                    if in_a_row.is_positive()
                    {
                        if column == 5 || pieces[row][column+1] != 'o'
                        {
                            empty_sides += 1;
                            score += in_a_row * empty_sides;
                            in_a_row = 0;
                        }
                    }
                    empty_sides = 1;
                },

                _ => println!("wtf")
            }
        }

        if in_a_row.is_positive() && empty_sides.is_positive()
        {
            if in_a_row == 4 { score = 128 }
            score += in_a_row * empty_sides;
        }

        in_a_row = 0;
        empty_sides = 0;
    }

score

}

//return array with all spots that can be played
fn generate_legal_moves(pieces: [[char; 6]; 7])->[bool; 7]
{
        
    let mut x: [bool; 7] = [false; 7];

    for i in 0..6
    {
        if (pieces[6][i]) == ' '
        {
            x[i] = true;
        }
    }

    x
}
