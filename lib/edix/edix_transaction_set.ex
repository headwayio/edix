defmodule Edix.EdixTransactionSet do
  defstruct transaction_set_control_number: "",
            transaction_set_identifier_code: "",
            transaction_set_date: "",
            transaction_set_time: "",
            transaction_set_segments: []
end
