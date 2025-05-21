import os
import re
import shutil

def extract_error_message(log_file):
    """Extracts the 4th line (error message) from a log file."""
    try:
        with open(log_file, 'r', encoding='utf-8', errors='ignore') as f:
            lines = f.readlines()
            if len(lines) >= 4:
                return lines[3].strip()  # 4th line (index 3)
            else:
                return None #handle files that don't have 4 lines
    except FileNotFoundError:
        return None  # Handle missing file
    except Exception as e:
        print(f"Error reading {log_file}: {e}")
        return None

def find_unique_errors(root_dir):
    """Finds unique errors in verbose-log.txt files under a root directory."""
    error_map = {}  # {error_message: [list of directory paths]}

    for dirpath, _, filenames in os.walk(root_dir):
        if "failed" in dirpath or "unrecognized" in dirpath:
            continue
        if "verbose-log.txt" in filenames:
            log_file = os.path.join(dirpath, "verbose-log.txt")
            error_message = extract_error_message(log_file)

            if error_message:
                if error_message not in error_map:
                    error_map[error_message] = []
                error_map[error_message].append(dirpath)

    return error_map

def move_directories_to_new_dir(target_dir, destination_dir, directories_to_move):
    """
    Creates a directory and moves the specified subdirectories into it.

    Args:
        target_dir (str): The path to the new directory to be created.
        destination_dir (str): The directory where the new directory will be created.
        directories_to_move (list): A list of directory names to move.
    """

    # 1. Create the target directory:
    target_path = os.path.join(destination_dir, target_dir)
    try:
        os.makedirs(target_path, exist_ok=True)  # Create if doesn't exist
        print(f"Created directory: {target_path}")
    except Exception as e:
        print(f"Error creating directory {target_path}: {e}")
        return False # return false if directory could not be created


    # 2. Move directories into it:
    for dir_name in directories_to_move:
        source_path = os.path.join(destination_dir, dir_name)
        if os.path.isdir(source_path):
            try:
                shutil.move(source_path, target_path)
                print(f"Moved directory '{dir_name}' to '{target_path}'")
            except Exception as e:
                print(f"Error moving directory '{dir_name}': {e}")
                return False # return false if a directory failed to move
        else:
            print(f"Warning: '{dir_name}' is not a directory or does not exist")

    return True # if everything went smoothly, return true


def print_unique_errors(root_directory, error_map):
    """Prints unique errors and their corresponding directories."""
    if not error_map:
         print("No errors found")
         return

    count = 1
    print("Unique Errors and their Directories:")
    for error, dirs in error_map.items():
        print("-" * 40)
        print(f"Error: {error}")
        print(f"  Found in directories:")
        for dir in dirs:
            print(f"    - {dir}")
        print()
        move_directories_to_new_dir(f"error_{count}", root_directory, dirs)
        count += 1

if __name__ == "__main__":
    root_directory = input("Enter the root directory to search: ")

    if not os.path.isdir(root_directory):
        print("Error: Provided directory not found or is not a directory.")
    else:
        error_map = find_unique_errors(root_directory)
        print_unique_errors(root_directory, error_map)
