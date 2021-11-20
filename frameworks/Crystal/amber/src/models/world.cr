class World < Jennifer::Model::Base
  table_name "World"

  mapping(
    id: Primary32,
    random_number: {type: Int32, column: "randomnumber"},
  )
end
