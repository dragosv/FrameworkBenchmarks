class Fortune < Jennifer::Model::Base
  table_name "Fortune"

  mapping(
    id: Primary32,
    message: String
  )
end
