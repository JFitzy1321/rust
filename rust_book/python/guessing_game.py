'''
A random number guessing game wriiten in Python3.8
I wrote this mainly to constract python code to rust code
'''
import random


def get_input(secret_num: int) -> int:
    while True:
        _user_input = input("\nEnter a number: ")
        if (_user_input == "printnum"):
            print(secret_num)
            continue
        try:
            return int(_user_input)  # parse string to int
        except ValueError:
            print("Please enter only whole numbers!")
            continue


def game():
    secret_num = random.randint(1, 100)
    while True:
        guess = get_input(secret_num)

        print(f"You guessed {guess}. ", end="")  # no return character
        if (guess < secret_num):
            print("Number too small!")
        elif (guess > secret_num):
            print("Number too big!")
        else:
            print("You win!!")
            break


if __name__ == '__main__':
    print("Number Guessing Game written in Python")
    game()
