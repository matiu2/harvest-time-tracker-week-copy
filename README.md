A bunch of programs to re-post my last week's harvest time sheet.

# To configureo

Create a .env file with these variables:

# Your harvest API token
token=XXXX
# Your account number 
account=YYYY


# To repost: 

# Download all time entries into `data.json`
RUST_LOG=info cargo run --bin download # Downloads all the time sheet entries

# Extract a week of dates into `source_week.json`
# Date format is: 2021-12-31 (yyyy-mm-dd)
RUST_LOG=info cargo run --bin extract  start-date end-date

# Change that week into the new week you want to post to in `to_upload.json`
RUST_LOG=info cargo run --bin map_week new-start-date

# Post the new week through the API
RUST_LOG=info cargo run --bin upload
