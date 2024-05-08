defmodule Edix.EdixFunctionalGroup do
  defstruct functional_group_control_number: "",
            application_sender_code: "",
            application_receiver_code: "",
            date: "",
            time: "",
            group_reference_number: "",
            transaction_set_control_number: "",
            transaction_sets: []
end
