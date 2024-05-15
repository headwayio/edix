defmodule EdixTest do
  use ExUnit.Case

  # alias Edix.EdixDocument

  doctest Edix

  test "parse edi file" do
    {:ok, edix_document} = Edix.parse_file("./sample.edi")

    assert edix_document.element_delimiter == "*"
    assert edix_document.segment_delimiter == "~"
    assert edix_document.sub_element_delimiter == ">"

    [envelope] = edix_document.envelope
    assert envelope.authorization_information == ""
    assert envelope.authorization_qualifier == "00"
    assert length(envelope.functional_groups) == 1

    [functional_group] = envelope.functional_groups
    assert functional_group.functional_group_control_number == "SM"
    assert functional_group.application_sender_code == "915792US00"
    assert functional_group.application_receiver_code == "8TMW"
    assert functional_group.date == "20240429"
    assert functional_group.time == "0614"
    assert functional_group.group_reference_number == "SM"
    assert functional_group.transaction_set_control_number == "SM"

    [transaction_set] = functional_group.transaction_sets

    assert transaction_set.transaction_set_control_number == "0001"
    assert transaction_set.transaction_set_identifier_code == "204"
    assert transaction_set.transaction_set_name == "Motor Carrier Load Tender"
    assert transaction_set.transaction_set_date == "transaction_set.date.to_string()"

    assert transaction_set.transaction_set_time ==
             "transaction_set.transaction_set_time.to_string()"

    assert length(transaction_set.transaction_set_segments) == 32
  end

  test "parse edi string" do
    edi_string = File.read!("./sample.edi")

    {:ok, edix_document} = Edix.parse(edi_string)

    assert edix_document.element_delimiter == "*"
    assert edix_document.segment_delimiter == "~"
    assert edix_document.sub_element_delimiter == ">"

    [envelope] = edix_document.envelope
    assert envelope.authorization_information == ""
    assert envelope.authorization_qualifier == "00"
    assert length(envelope.functional_groups) == 1

    [functional_group] = envelope.functional_groups
    assert functional_group.functional_group_control_number == "SM"
    assert functional_group.application_sender_code == "915792US00"
    assert functional_group.application_receiver_code == "8TMW"
    assert functional_group.date == "20240429"
    assert functional_group.time == "0614"
    assert functional_group.group_reference_number == "SM"
    assert functional_group.transaction_set_control_number == "SM"

    [transaction_set] = functional_group.transaction_sets

    assert transaction_set.transaction_set_control_number == "0001"
    assert transaction_set.transaction_set_identifier_code == "204"
    assert transaction_set.transaction_set_name == "Motor Carrier Load Tender"
    assert transaction_set.transaction_set_date == "transaction_set.date.to_string()"

    assert transaction_set.transaction_set_time ==
             "transaction_set.transaction_set_time.to_string()"

    assert length(transaction_set.transaction_set_segments) == 32
  end
end
