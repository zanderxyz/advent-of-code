use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day20.txt");

#[derive(Clone, Debug)]
struct Input {
    numbers: Vec<isize>,
}

impl Input {
    fn new(input: &str) -> Input {
        let numbers = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();

        Input { numbers }
    }
}

#[derive(Debug, Clone)]
struct DoublyLinkedList<const N: usize> {
    numbers: [isize; N],
    // The index of the next element after the one at this index
    next_ptr: [usize; N],
    // The index of the previous element
    prev_ptr: [usize; N],
}

impl<const N: usize> From<Vec<isize>> for DoublyLinkedList<N> {
    fn from(numbers: Vec<isize>) -> Self {
        Self {
            numbers: numbers.try_into().unwrap(),
            next_ptr: (0..N)
                .map(|i| (i + 1) % N)
                .collect_vec()
                .try_into()
                .unwrap(),
            prev_ptr: (0..N)
                .map(|i| (i + N - 1) % N)
                .collect_vec()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const N: usize> DoublyLinkedList<N> {
    fn get(&self, index: usize) -> isize {
        self.numbers[index % self.numbers.len()]
    }

    fn in_order(&self) -> Vec<isize> {
        let mut output = Vec::new();
        let mut i = 0;

        for _ in 0..self.numbers.len() {
            output.push(self.numbers[i]);
            i = self.next_ptr[i];
        }
        output
    }

    fn grove(&self) -> isize {
        let len = self.numbers.len();
        let numbers = self.in_order();
        let zero_position = numbers.iter().find_position(|n| **n == 0).unwrap().0;

        let offset1 = (zero_position + 1000) % len;
        let offset2 = (zero_position + 2000) % len;
        let offset3 = (zero_position + 3000) % len;
        numbers[offset1] + numbers[offset2] + numbers[offset3]
    }

    fn mix(&mut self) {
        for index in 0..self.numbers.len() {
            let distance = self.get(index);
            self.move_by(index, distance);
        }
    }

    fn set_pointer_from_to(&mut self, from: usize, to: usize) {
        self.next_ptr[from] = to;
        self.prev_ptr[to] = from;
    }

    fn move_by(&mut self, index: usize, distance: isize) {
        let len = self.numbers.len() as isize;
        let mut target = index;
        let mut distance = distance % (len - 1);
        if distance != 0 {
            // "Lift up" the current number, so it's out of the list
            let prev = self.prev_ptr[index];
            let next = self.next_ptr[index];
            self.set_pointer_from_to(prev, next);

            // Move backwards or forwards to find the new index position
            if distance > 0 {
                while distance > 0 {
                    target = self.next_ptr[target];
                    distance -= 1;
                }
                target = self.next_ptr[target];
            } else {
                while distance < 0 {
                    target = self.prev_ptr[target];
                    distance += 1;
                }
            }

            // Insert the number back into the list right before this spot
            let prev = self.prev_ptr[target];
            self.set_pointer_from_to(prev, index);
            self.set_pointer_from_to(index, target);
        }
    }

    fn decrypt(&mut self, key: isize) {
        for number in self.numbers.iter_mut() {
            *number *= key;
        }
    }
}

fn part1<const N: usize>(input: &Input) -> isize {
    let mut cycle: DoublyLinkedList<N> = input.numbers.clone().into();
    cycle.mix();
    cycle.grove()
}

fn part2<const N: usize>(input: &Input) -> isize {
    let mut cycle: DoublyLinkedList<N> = input.numbers.clone().into();
    cycle.decrypt(811589153);
    for _ in 0..10 {
        cycle.mix();
    }
    cycle.grove()
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1::<5000>(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2::<5000>(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day20.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1::<7>(&input), 3);
        assert_eq!(part2::<7>(&input), 1623178306);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1::<5000>(&input), 14526);
        assert_eq!(part2::<5000>(&input), 9738258246847);
    }
}
