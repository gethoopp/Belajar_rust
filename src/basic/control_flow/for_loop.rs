


pub fn contohhLopp() {
   
    let mut array: [i32;5] = [1,2,3,4,5];
    
    for value in array {
       println!("ini index ke : {}", value);

    }

   

}




pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
  nums.sort();
for value in 0..=nums.len(){

    
    
      for value in 0..=nums.len() {
        if nums[value] == nums[value + 1] {
                return true;
            }


            let mut j = value - 1;

            while j < value {
                j += 1; 
                if nums[value] == nums[value + 1] {
                    return false;
                }
            }
      }
}
        return false;
    } 


   pub fn find_missing(a: Vec<i32>, b: Vec<i32>) -> i32 {
      let mut hasil = 0; 
        for value in a {
           hasil ^= value;

          
        }

         for value in &b {
            hasil ^= value;
           }

        return hasil;
        

    }




    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    // }
    


   
    

    