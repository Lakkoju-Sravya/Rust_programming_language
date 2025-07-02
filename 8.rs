
/*
Write a program to create an array of 10 elements and implement the following
a. Create a of 2nd and 3rd element
b. Omit the start index of the slice
c. Omit the End Index of the Slice
d. Omit both Start and End Index of the Slice
*/

fn main() {
    let arr =[1,2,3,4,5,6,7,8,9];
    let slice_2nd_3rd = &arr[1..3];
    println!("a. Slice of 2nd and 3rd element: {:?}", slice_2nd_3rd);
    let slice_omit_end = &arr[..7];
    println!("c. Omit the end index of the slice: {:?}", slice_omit_end);
    let slice_omit_both = &arr[..];
    println!("d. Omit both start and end index of the slice: {:?}", slice_omit_both);
}
