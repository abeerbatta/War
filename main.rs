#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

fn deal(shuf: &[u8; 52]) -> [u8; 52] {
    let mut firstplayer = Vec::new(); // Initialize Player 1's deck
    let mut secondplayer = Vec::new(); // Initialize Player 2's deck
    let mut temp = Vec::new(); // Initialize a temporary deck for holding cards during a "war"
    let mut rev_shuf = Vec::new();
    rev_shuf = shuf.to_vec();
    rev_shuf.reverse(); // Reverse the shuffled deck

    // Distribute cards to players
    for i in 0..rev_shuf.len() {
        if i % 2 == 0 {
            firstplayer.push(rev_shuf[i]); // Give card to Player 1
        } else {
            secondplayer.push(rev_shuf[i]); // Give card to Player 2
        }
    }

    // Replace all Aces (1's) with 14's to make comparisons easier
    for i in firstplayer.iter_mut() {
        if *i == 1 {
            *i = 14;
        }
    }
    for i in secondplayer.iter_mut() {
        if *i == 1 {
            *i = 14;
        }
    }

    // Main game loop
    while firstplayer.len() > 0 && secondplayer.len() > 0 {
        // Player 1 wins the round
        if firstplayer[0] > secondplayer[0] {
            temp.push(firstplayer[0]);
            temp.push(secondplayer[0]);
            temp.sort_by(|a, b| b.cmp(a));
            firstplayer.extend(temp.drain(..));
        }
        // Player 2 wins the round
        else if firstplayer[0] < secondplayer[0] {
            temp.push(firstplayer[0]);
            temp.push(secondplayer[0]);
            temp.sort_by(|a, b| b.cmp(a));
            secondplayer.extend(temp.drain(..));
        }
        // "War" occurs
        else if firstplayer.len() >= 2 && secondplayer.len() >= 2 {
            temp.push(firstplayer[0]);
            temp.push(secondplayer[0]);
            temp.push(firstplayer[1]);
            temp.push(secondplayer[1]);
            temp.sort_by(|a, b| b.cmp(a));
            firstplayer.remove(1);
            secondplayer.remove(1);
        }
        // Player 1 has only 1 card left and a "war" occurs
        else if firstplayer.len() == 1 && secondplayer.len() > 1 {
            if firstplayer[0] == secondplayer[0] {
                secondplayer.push(firstplayer[0]);
                secondplayer.push(secondplayer[0]);
            }
        }
        // Player 2 has only 1 card left and a "war" occurs
        else if secondplayer.len() == 1 && firstplayer.len() > 1 {
            if firstplayer[0] == secondplayer[0] {
                firstplayer.push(firstplayer[0]);
                firstplayer.push(secondplayer[0]);
            }
        }

        firstplayer.remove(0); // Discard the top card of Player 1's deck
        secondplayer.remove(0); // Discard the top card of Player 2's deck
    }
        
        if firstplayer.len()==0 && secondplayer.len()==0 // Case where there is a tie
        {
            temp.sort_by(|a, b| b.cmp(a));
            let mut p3=[0;52]; 
            for i in temp.iter_mut() // Replacing all the 14's to 1's
            {
                if *i == 14 
                {
                    *i = 1;
                }
            }
            for i in 0..temp.len() // Copying all the cards to array from vector since the return type is array
            {
             p3[i]=temp[i];
            }
            p3 // Returning the Winning deck
        }
        else if firstplayer.len()==0 // Case where Player2 Wins
        {
            temp.sort_by(|a, b| b.cmp(a));
            secondplayer.extend(temp); 
            let mut p2=[0;52]; 
            for i in secondplayer.iter_mut() // Replacing all the 14's to 1's
             {
                if *i == 14
                 {
                    *i = 1;
                }
            }
            for i in 0..secondplayer.len() // Copying all the cards to array from vector since the return type is array
            {
             p2[i]=secondplayer[i];
            }
            p2 // Returning the Winning deck
        }
        else  // Case where Player1 Wins
        {
            temp.sort_by(|a, b| b.cmp(a));
            firstplayer.extend(temp); 
            let mut p1=[0;52]; 
            for i in firstplayer.iter_mut() // Replacing all the 1's to 14's
             {
                if *i == 14 
                {
                    *i = 1;
                }
            }
            for i in 0..firstplayer.len() // Copying all the cards to array from vector since the return type is array
            {
             p1[i]=firstplayer[i];
            }
            p1 // Returns the winning deck
            
        }
}


    


#[cfg(test)]
#[path = "tests.rs"]
mod tests;

