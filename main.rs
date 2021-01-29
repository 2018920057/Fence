use std::io;
use std::cmp;
fn main() {
  let mut fence_str = String::new();
  let mut fence:Vec<i32> = Vec::new();
  io::stdin().read_line(&mut fence_str).expect("error");
  for spl in fence_str.split_whitespace(){
    fence.push(spl.parse().expect("parsing error"));
  }
  println!("{:?}",fence);
  println!("{}",solve(&fence));
}

//판자의 배열을 받아 최대 넓이 반환
fn solve(fence : &Vec<i32>) -> i32 {
  let length : usize= fence.len();
  //기저 사례: 판자의 너비가 1일 때
  if length == 1 {return fence[0]}
  //fence[..mid] 와 fence[mid+1..]로 문제 분할
  let mid = length/2;
  let mut ret = cmp::max(solve(&fence[..mid].to_vec()),solve(&fence[mid..].to_vec()));
  //둘 사이에 걸치는 판자가 제일 클 경우. 초깃값:cmp::min(fence[mid]+fence[mid+1])*2
  let mut lo = mid-1;
  let mut hi = mid;
  let mut height = cmp::min(fence[lo],fence[hi]); 
  while(lo>=0 && hi<length-1){
    if (hi<length-1 && (lo == 0 || fence[lo-1] < fence[hi+1])){
      hi += 1;
      height = cmp::min(height, fence[hi]);
    }else{
      lo -= 1;
      height = cmp::min(height, fence[lo]);
    }
    //확장 후 사각형의 넓이
    ret = cmp::max(ret, height * ((hi-lo+1)as i32));
  }
  ret
}