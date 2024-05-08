defmodule Edix.EdixInterchangeControl do
  defstruct interchange_control_number: "",
            sender_id: "",
            receiver_id: "",
            authorization_qualifier: "",
            authorization_information: "",
            security_qualifier: "",
            functional_groups: []
end
