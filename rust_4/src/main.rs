use std::fs;
fn main() {
    let contents = read_in_input();

    println!("With text:{:?}", contents);

    let iterrator_lines = contents.lines();

    let mut total_overlap = 0;
    let mut any_overlap = 0;

    for line in iterrator_lines{
        let mut ranges= line.split(",");
        // Access first and second range
        let first_range_str = ranges.next().unwrap();
        let second_range_str = ranges.next().unwrap();

        // create ranges from the strings
        let first_range_vec: Vec<i32> = first_range_str.split("-")
                                             .map(|x| x.parse::<i32>().unwrap())
                                             .collect();
        let second_range_vec: Vec<i32> = second_range_str.split("-")
                                             .map(|x| x.parse::<i32>().unwrap())
                                             .collect();
        let first_range = first_range_vec[0]..first_range_vec[1];

        let second_range = second_range_vec[0]..second_range_vec[1];
        println!("first range: {:?}", first_range);
        println!("Second range: {:?}", second_range);

        println!("Lowest value: {}", second_range.start.max(first_range.start));
        println!("Highest value: {}", second_range.end.min(first_range.end));
        
        let intersecting_elements = (second_range.start.max(first_range.start)..(second_range.end.min(first_range.end))+1).collect::<Vec<i32>>();

        println!("Intersecting elements: {:?}", intersecting_elements);

        let number_intersecting_elements = intersecting_elements.len();

        println!("Nr intersecting elements: {:?}", number_intersecting_elements);
        println!("Len first range {:?}", first_range.len());
        println!("Len second range {:?}", second_range.len());

        if number_intersecting_elements == second_range.len()+1 || number_intersecting_elements == first_range.len()+1 {
            total_overlap += 1;
        }
        if number_intersecting_elements > 0 {
            any_overlap += 1;
        }
    }
    println!("Total overlapping: {:?}", total_overlap);
    println!("Any overlapping: {:?}", any_overlap);
}   



fn read_in_input() -> String {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}
