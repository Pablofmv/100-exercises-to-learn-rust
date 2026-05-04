// TODO: Replace `todo!()`s with the correct implementation.
//  Implement `IntoIterator` for `&TicketStore`. The iterator should yield immutable
//  references to the tickets, ordered by their `TicketId`. (ok)
//  Implement additional traits on `TicketId` if needed.

use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use ticket_fields::{TicketDescription, TicketTitle};
use std::collections::btree_map;

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
}

impl <'a>IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = btree_map::Values<'a, TicketId, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.values()
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketId, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for i in 0..n_tickets {
            let draft = TicketDraft {
                title: ticket_title(),
                description: ticket_description(),
            };
            let id = store.add_ticket(draft.clone());
            let ticket = &store[id];
            assert_eq!(draft.title, ticket.title);
            assert_eq!(draft.description, ticket.description);
            assert_eq!(ticket.status, Status::ToDo);

            let ticket = &mut store[id];
            ticket.status = Status::InProgress;

            let ticket = &store[id];
            assert_eq!(ticket.status, Status::InProgress);
        }

        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}




async fn delete_user(pool: &PgPool,user_id: i32) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let res1 = sqlx::query!(
        "DELETE FROM orders WHERE user_id = $1",
        user_id
    )
    .execute(&mut *tx)
    .await?;

    let res2 = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(&mut *tx)
    .await?;

    if res2.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound)
    }

    tx.commit().await?;

    Ok(())

}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Insuficient Balance")]
    InsuficientBalance,

    #[error("Account Not Found")]
    AccountNotFound,

    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),
}

async fn transfer_50(pool: &PgPool,from_id: i32,to_id: i32,) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;

    let res1 = sqlx::query!(
        "UPDATE accounts SET balance = balance - 50 WHERE id = $1 AND balance >= 50",
        from_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e|{
        AppError::DatabaseError(e)
    })?;

    if res1.rows_affected() == 0 {
        return Err(AppError::InsufficientBalance);
    }

    let res2 = sqlx::query!(
        "UPDATE accounts SET balance = balance + 50 WHERE id = $1",
        to_id
    )
    .execute(&mut *tx)
    .await?;

    if res2.rows_affected() == 0 {
        return Err(AppError::AccountNotFound);
    }

    tx.commit().await?;

    Ok(())
}
