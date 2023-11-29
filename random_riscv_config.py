# Generate a random (valid) riscv config
import random


def main():
    xlen = random.choice([32, 64])
    extensions = random.choice(
        ["g", "gc", "gcv"]
    )
    add_ons = random.choice(
        ["", "_zvl128b", "_zvl256b", "_zvl512b", "_zvl1028b", "_zicond"]
    )

    print(f"rv{xlen}{extensions}{add_ons}")


if __name__ == "__main__":
    main()
