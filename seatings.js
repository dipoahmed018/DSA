const plane_seats_input = [
  [3, 2],
  [4, 3],
  [2, 3],
  [3, 4]
];
const passanger = 30;

//processing the input
const window_seats = [];
const aisle_seats = [];
const middle_seats = [];

const parsed_seats = new Set();
let time = 1;
let current_row = 1;
while (parsed_seats.size != plane_seats_input.length) {
  for (let index = 0; index < plane_seats_input.length; index++) {
    const element = plane_seats_input[index];
    const seat_column = element[0];
    const seat_row = element[1];
    console.log(time, current_row)
    time++
    if(seat_row < current_row) {
      parsed_seats.add(index);
      continue;
    }

    // window sets
    if(index == 0 || index == plane_seats_input.length - 1) {
      window_seats.push({passanger: null});
    }

    //aisle seats 
    if((index == 0 || index == plane_seats_input.length - 1) && seat_column > 1) {
      aisle_seats.push({passanger: null});
    } else if(seat_column > 1) {
      aisle_seats.push({passanger: null}, {passanger: null})
    } else {
      aisle_seats.push({passanger: null});

    }
    
    //middle seats 
    for (let i = 0; i < seat_column - 2; i++) {
      middle_seats.push({passanger: null})
    }
  }
  current_row++;
}


let next_window_seats = 0;
let next_aisle_seats = 0;
let next_middle_seats = 0;
for (let PID = 1; PID <= passanger; PID++) {
  if(aisle_seats[next_aisle_seats]) {
    aisle_seats[next_aisle_seats].passanger = PID;
    next_aisle_seats++
  } else if(window_seats[next_window_seats]) {
    window_seats[next_window_seats].passanger = PID;
    next_window_seats++
  } else if(middle_seats[next_middle_seats]) {
    middle_seats[next_middle_seats].passanger = PID;
    next_middle_seats++
  } else {
    break;
  }
}

console.log(current_row, 'row')