use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/herodotus", NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS optimism (
            id              SERIAL PRIMARY KEY,
            output_root     VARCHAR NOT NULL,
            l1_output_index INTEGER NOT NULL,
            l2_blocknumber  INTEGER NOT NULL,
            l1_timestamp    INTEGER NOT NULL
            )
    ",
    )?;

    Ok(())
}
