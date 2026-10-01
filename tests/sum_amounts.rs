use sum_amounts::{settle, Error, Line, Obligation, Settlement};

fn ob(id: u64, amount_due: u64, priority: u64) -> Obligation {
    Obligation {
        id,
        amount_due,
        priority,
    }
}

fn settlement(unapplied: u64, pairs: &[(u64, u64)]) -> Settlement {
    Settlement {
        lines: pairs
            .iter()
            .map(|(id, applied)| Line {
                id: *id,
                applied: *applied,
            })
            .collect(),
        unapplied,
    }
}

fn assert_settlement(payment: u64, obligations: &[Obligation], unapplied: u64, pairs: &[(u64, u64)]) {
    assert_eq!(
        settle(payment, obligations).unwrap(),
        settlement(unapplied, pairs)
    );
}

#[test]
fn empty() {
    assert_settlement(100, &[], 100, &[]);
}

#[test]
fn pays_in_full() {
    assert_settlement(
        500,
        &[ob(1, 100, 1), ob(2, 40, 3), ob(3, 60, 2)],
        300,
        &[(1, 100), (2, 40), (3, 60)],
    );
}

#[test]
fn higher_priority_is_paid_first() {
    assert_settlement(
        60,
        &[ob(1, 80, 1), ob(2, 50, 9), ob(3, 25, 9)],
        0,
        &[(1, 0), (2, 50), (3, 10)],
    );
}

#[test]
fn same_priority_pays_earlier_input_first() {
    assert_settlement(
        10,
        &[ob(1, 50, 1), ob(2, 30, 1), ob(3, 20, 1)],
        0,
        &[(1, 10), (2, 0), (3, 0)],
    );
}

#[test]
fn remainder_spills_to_the_next_priority() {
    assert_settlement(
        120,
        &[ob(1, 100, 1), ob(2, 40, 5), ob(3, 60, 5)],
        0,
        &[(1, 20), (2, 40), (3, 60)],
    );
}

#[test]
fn duplicate_id() {
    let err = settle(5, &[ob(1, 10, 1), ob(1, 4, 2)]).unwrap_err();
    assert_eq!(err, Error::DuplicateId);
}
