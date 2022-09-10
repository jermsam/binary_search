fn main() {
    println!("Hello, world!");
    let sorted_list:[i8;10] = [0,10,20,30,40,50,60,70,80,90];
    let target:i8 = 11;
    verify(&sorted_list, &target,&binary_search);
    let target:i8 = 0;
    verify(&sorted_list, &target,&binary_search);
}

// ideal for short unsorted lists
fn binary_search (list:&[i8; 10], target: &i8) -> Option<i8> {
    /* Returns index position if found else returns None */
    let mut first:usize = 0;
    let mut last:usize = list.len() - 1 ;
    let mut index:  Option<i8> =  None;
    while first <= last {
        let  midpoint = (first + last) / 2;
        if *target < list[midpoint as usize] {
            last = (midpoint - 1) as usize;
        } else if  *target > list[midpoint]  {
            first = (midpoint + 1) as usize;
        }
        else {
            index = Some(midpoint as i8);
            break
        }
    }
   index
}

fn verify(list:&[i8; 10], target: &i8, search: &dyn Fn(&[i8; 10], &i8) -> Option<i8>) {
    let index: Option<i8> = search(&list, &target);
    match index {
        None => {
            println!("There is no match for {} in {:?}.", target, *list);
        },
        Some(i) => {
            println!("Position of {} in {:?} is {:?}", target, *list,i);
        },
    }
}

/*
fn main() {
    println!("Hello, world!");
    let sorted_list:[i8;10] = [0,1,2,3,4,5,6,7,8,9];
    let target:i8 = 11;
    binary_search(&sorted_list, &target);
    let target:i8 = 0;
    binary_search(&sorted_list, &target);

}

// ideal for short unsorted lists
fn binary_search (list:&[i8; 10], target: &i8){
    /* Returns index position if found else returns None */
    let mut first:usize = 0;
    let mut last:usize = list.len() - 1 ;
    let mut index:  Option<i8> =  None;
    while first <= last {
        let  midpoint = (first + last) / 2;
        if *target < list[midpoint as usize] {
            last = (midpoint - 1) as usize;
        } else if  *target > list[midpoint]  {
            first = (midpoint + 1) as usize;
        }
        else {
            index = Some(midpoint as i8);
            break
        }
    }

    match index {
        None => {
            println!("There is no match for {} in {:?}.", target, *list);
        },
        Some(i) => {
            println!("Position of {} in {:?} is {:?}", target, *list,i);
        },
    }
}
*/
