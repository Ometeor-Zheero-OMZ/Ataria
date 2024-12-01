import { FC, useEffect } from "react";
import {
  SortableContext,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { ColumnProps, TaskParam } from "@/types/type";
import Task from "@/components/Task/Task";

const Column: FC<ColumnProps> = ({ tasks }) => {
  return (
    <div className="bg-[#f2f2f3] rounded-md p-4 w-4/5 max-w-md flex flex-col gap-4">
      <SortableContext items={tasks} strategy={verticalListSortingStrategy}>
        {tasks.map((task) => (
          <Task key={task.id} task={task} />
        ))}
      </SortableContext>
    </div>
  );
};

export default Column;
