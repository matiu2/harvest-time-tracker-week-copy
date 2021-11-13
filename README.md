A bunch of programs to re-post my last week's harvest time sheet.

# To configure

Create a .env file with these variables:

    # Your harvest API token
    token=XXXX
    # Your harvest account number
    account=YYYY

To get your token, log in to your harvest app, then go here: https://id.getharvest.com/developers

To get your account ID / account number, go here: https://id.getharvest.com/ -- then hover over or right click and `Copy Link` one of the account links and see the last part of the URL. (Just clicking it may cause a redirect to some other URL).
# To repost:
## Download all time entries into `data.json`

    RUST_LOG=info cargo run --bin download # Downloads all the time sheet entries

## Extract a week of dates into `source_week.json`

Date format is: 2021-12-31 (yyyy-mm-dd)

    RUST_LOG=info cargo run --bin extract  start-date end-date

## Change that week into the new week you want to post to in `to_upload.json`

    RUST_LOG=info cargo run --bin map_week new-start-date

## Post the new week through the API

    RUST_LOG=info cargo run --bin upload
