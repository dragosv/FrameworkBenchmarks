class CreateWorld < Jennifer::Migration::Base
  def up
    create_table :world do |t|
      t.integer :randomnumber, {:null => false}
    end
  end

  def down
    drop_table :world if table_exists? :world
  end
end
