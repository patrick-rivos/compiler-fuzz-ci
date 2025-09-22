import pandas as pd
import matplotlib.pyplot as plt
import requests
import re
import json

def pull_data():
    url = 'https://gcc.gnu.org/bugzilla/buglist.cgi?bug_status=UNCONFIRMED&bug_status=NEW&bug_status=ASSIGNED&bug_status=SUSPENDED&bug_status=WAITING&bug_status=REOPENED&bug_status=RESOLVED&bug_status=VERIFIED&bug_status=CLOSED&cf_known_to_fail_type=allwords&cf_known_to_work_type=allwords&f1=longdesc&list_id=489840&o1=substring&query_format=advanced&resolution=---&resolution=FIXED&resolution=INVALID&resolution=WONTFIX&resolution=DUPLICATE&resolution=WORKSFORME&resolution=MOVED&v1=found%20via%20fuzzer&ctype=csv&human=1'
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(f"Failed to fetch data: {response.status_code}")
    with open('temp-fuzzer-find-reports.csv', 'w') as f:
        f.write(response.text)

    return pd.read_csv('temp-fuzzer-find-reports.csv', sep=',').applymap(lambda x: x.strip() if isinstance(x, str) else x)

def combine_data(old_df, new_df):
    combined_df = pd.concat([old_df, new_df]).drop_duplicates().reset_index(drop=True)
    return combined_df

def filter_data(df, filter):
    pattern = '|'.join(map(re.escape, filter))
    filtered_df = df[df['Summary'].str.contains(pattern, case=False, na=False)]
    print(f"Filtered DataFrame with pattern '{pattern}':")
    return filtered_df

def write_links(df, filename):
    with open(filename, 'w') as f:
        for bug_id in df['Bug ID'].sort_values().tolist():
            f.write(f"1. https://gcc.gnu.org/bugzilla/show_bug.cgi?id={bug_id}\n")

def main():
    # Read the CSV file into a DataFrame
    df = pd.read_csv('fuzzer-find-reports.csv', sep=',').applymap(lambda x: x.strip() if isinstance(x, str) else x)
    temp_df = pull_data()
    temp_df = temp_df[~temp_df['Resolution'].isin(['DUPL', 'DUPLICATE'])]
    df = combine_data(df, temp_df)
    df = df[~df['Resolution'].isin(['DUPL', 'DUPLICATE'])]

    with open("../README.md", "r") as f:
        readme = f.read()

    bugzillas = [line for line in readme.split("\n") if line.startswith("1. https://gcc.gnu.org/bugzilla/show_bug.cgi?id=")]
    print(bugzillas)

    bugzilla_ids = [int(line.split("=")[-1].strip()) for line in bugzillas]
    print(bugzilla_ids)
    print(len(bugzilla_ids))

    temp_df_ids = temp_df['Bug ID'].tolist()
    print(len(set(temp_df_ids) | set(bugzilla_ids)))

    filtered_df = df[df['Bug ID'].isin(set(temp_df_ids) | set(bugzilla_ids))]
    filtered_df = filtered_df[~filtered_df['Resolution'].isin(['DUPL', 'DUPLICATE', 'INVA', 'INVALID'])]

    filtered_df = filtered_df.drop_duplicates(subset=['Bug ID'])
    filtered_df['timestamp'] = pd.to_datetime(filtered_df['Opened'])
    filtered_df.set_index('timestamp', inplace=True)
    print(filtered_df)
    filtered_df.sort_values('Bug ID').to_csv("filtered-bugzilla-reports.csv")
    print(f"size of filtered df: {len(filtered_df)}")
    print(len(set(filtered_df['Bug ID'].tolist())))

    miscompiles = [
        "miscompile",
        "miscompilation",
        "incorrect code",
        "incorrect behavior",
        "wrong code",
        "wrong result",
        "incorrect result",
        "incorrect output",
        "wrong output",
        "mismatch",
        "runtime",
        "runtime error",
    ]

    miscompiled_df = filter_data(filtered_df, miscompiles)
    # special case some miscompile bugs that don't have the keywords
    miscompile_special_case_ids = [112801, 112855, 112932, 116033, 116035]
    miscompiled_special_case_df = filtered_df[filtered_df['Bug ID'].isin(miscompile_special_case_ids)]
    miscompiled_df = pd.concat([miscompiled_df, miscompiled_special_case_df]).drop_duplicates()
    print(f"size of miscompiled df: {len(miscompiled_df)}")
    write_links(miscompiled_df, "miscompiled-bugzilla-reports.md")

    miscompiled_set = set(miscompiled_df['Bug ID'].tolist())

    ices = [
        "internal compiler error",
        "ice",
        "segmentation fault",
        "segfault",
        "unrecognizable insn",
        "unrecognize",
        "undefined",
        "assertion",
        "crash",
        "abort",
        "core dump",
        "core dumped",
    ]

    ice_df = filter_data(filtered_df, ices)
    # special case some ice bugs that don't have the keywords
    ice_special_case_ids = [115143, 116280]
    ice_special_case_df = filtered_df[filtered_df['Bug ID'].isin(ice_special_case_ids)]
    ice_df = pd.concat([ice_df, ice_special_case_df]).drop_duplicates()
    ice_df.sort_values('Bug ID').to_csv("ice-bugzilla-reports.csv")
    write_links(ice_df, "ice-bugzilla-reports.md")
    print(f"size of ice df: {len(ice_df)}")

    ice_df_set = set(ice_df['Bug ID'].tolist())
    other_set = miscompiled_set | ice_df_set
    print(len(other_set))


    other_df = filtered_df[~filtered_df['Bug ID'].isin(other_set)]
    other_df.sort_values('Bug ID').to_csv("other-bugzilla-reports.csv")
    write_links(other_df, "other-bugzilla-reports.md")
    print(f"size of other df: {len(other_df)}")

    other_df_set = set(other_df['Bug ID'].tolist())

    daily_counts = filtered_df.resample('D').size()

    print(daily_counts)
    cumulative_counts = daily_counts.cumsum()
    print(cumulative_counts)
    plt.figure(figsize=(10, 6))
    plt.plot(cumulative_counts.index, cumulative_counts.values, linestyle='-')
    plt.title('Cumulative Number of Bugzilla Reports Over Time')
    plt.xlabel('Date')
    plt.ylabel('Cumulative Count of Reports')
    plt.grid(True)
    plt.tight_layout()
    plt.savefig('cumulative_bugzilla_reports.png')

if __name__ == "__main__":
    main()
