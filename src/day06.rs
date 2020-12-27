// This solution collects the inputs into hashsets. Although quite simple,
// that may be an unnecessary overhead. As we have a fixed amount of inputs to
// deal with, another option would be to treat an unsigned integer as a bit
// array and save the answers as its bits.

use std::{collections::HashSet, str::Lines};

use crate::helpers::read;

type DeclarationForm = HashSet<char>;

/// Assumptions
///
/// `input` is well behaved.
fn sum_anyone_yes(input: &str) -> usize {
    AnswersIterator::new(input, AnyoneAnswers)
        .map(|ga| ga.len())
        .sum::<usize>()
}

/// Assumptions
///
/// `input` is well behaved.
fn sum_everyone_yes(input: &str) -> usize {
    AnswersIterator::new(input, EveryoneAnswers)
        .map(|ga| ga.len())
        .sum::<usize>()
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let input = read::to_str("day06").unwrap();
    println!("Day 06");
    println!(
        "Sum of the questions answered yes by anyone: {}",
        sum_anyone_yes(&input)
    );
    println!(
        "Sum of questions answered yes by everyone: {}",
        sum_everyone_yes(&input)
    );
}

//--------------------------------------------------------------------
// Data Structures
//--------------------------------------------------------------------
trait AnswerType {
    fn add_to_group(&self, group_ans: &mut DeclarationForm, next_person_ans: &str);

    fn create_group(&self, first_answers: Option<&str>) -> Option<DeclarationForm> {
        if let Some(str) = first_answers {
            if str.is_empty() {
                None
            } else {
                let mut group = DeclarationForm::new();
                group.extend(str.chars());
                Some(group)
            }
        } else {
            None
        }
    }
}

struct AnyoneAnswers;

impl AnswerType for AnyoneAnswers {
    fn add_to_group(&self, group_ans: &mut DeclarationForm, next_answers: &str) {
        group_ans.extend(next_answers.chars());
    }
}

struct EveryoneAnswers;

impl AnswerType for EveryoneAnswers {
    fn add_to_group(&self, group_ans: &mut DeclarationForm, next_answers: &str) {
        *group_ans = group_ans
            .intersection(&next_answers.chars().collect::<DeclarationForm>())
            .cloned()
            .collect();
    }
}

// Iterator-----------------------------------------------------------

struct AnswersIterator<'a, A: AnswerType> {
    answer_type: A,
    inner_iter: Lines<'a>,
}

impl<'a, A> AnswersIterator<'a, A>
where
    A: AnswerType,
{
    pub fn new(input_str: &'a str, answer_type: A) -> Self {
        Self {
            answer_type,
            inner_iter: input_str.lines(),
        }
    }
}

impl<'a, A> Iterator for AnswersIterator<'a, A>
where
    A: AnswerType,
{
    type Item = DeclarationForm;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut group) = self.answer_type.create_group(self.inner_iter.next()) {
            while let Some(ans) = self.inner_iter.next() {
                if ans.is_empty() {
                    break;
                }
                self.answer_type.add_to_group(&mut group, ans);
            }
            Some(group)
        } else {
            None
        }
    }
}
