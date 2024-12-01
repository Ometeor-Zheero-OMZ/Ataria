"use client";

import { useEffect, useState } from "react";
import {
  closestCorners,
  DndContext,
  DragEndEvent,
  PointerSensor,
  TouchSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import { arrayMove } from "@dnd-kit/sortable";

import Column from "@/components/Column/Column";
import Input from "@/components/Input/Input";
import { TaskParam } from "@/types/type";
import { useRouter } from "next/navigation";
import { useAuth } from "@/context/AuthContext";
import { ERROR_MESSAGES } from "@/constants/message";
import Navbar from "@/components/Navbar";
import {
  addTask,
  changeTaskStatus,
  deleteTask,
  fetchTasks,
  updateTask,
} from "@/api/todos";

export default function Page() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [tasks, setTasks] = useState<TaskParam[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getTasks = async () => {
      if (!loading && !user) {
        router.push("/");
        return;
      }

      if (user) {
        try {
          const data = await fetchTasks(user.token);
          if (data && Array.isArray(data)) {
            setTasks(data);
          } else {
            setError(ERROR_MESSAGES.INVALID_DATA_FORMAT_NOT_ARRAY_MESSAGE);
          }
        } catch (err) {
          setError(ERROR_MESSAGES.FETCH_MISSION_FAILED_MESSAGE);
        } finally {
          setIsLoading(false);
        }
      }
    };

    getTasks();
  }, [loading, user, router]);

  const handleAddTask = async (title: string, description: string) => {
    if (!loading && !user) {
      router.push("/");
      return;
    }

    if (user) {
      try {
        await addTask(user.token, title, description);
        const data = await fetchTasks(user.token);

        if (data && Array.isArray(data)) {
          setTasks(data);
        } else {
          setError(ERROR_MESSAGES.INVALID_DATA_FORMAT_NOT_ARRAY_MESSAGE);
        }
      } catch (err) {
        setError(ERROR_MESSAGES.APPEND_MISSION_FAILED_MESSAGE);
      }
    }
  };

  const handleUpdateTask = async (updatedTask: TaskParam) => {
    if (!loading && !user) {
      router.push("/");
      return;
    }

    if (user) {
      try {
        await updateTask(user.token, updatedTask);
        setTasks((tasks) =>
          tasks.map((task) => (task.id === updatedTask.id ? updatedTask : task))
        );
      } catch (err) {
        setError(ERROR_MESSAGES.UPDATE_MISSION_FAILED_MESSAGE);
      }
    }
  };

  const handleDeleteTask = async (taskId: number) => {
    if (!loading && !user) {
      router.push("/");
      return;
    }

    if (user) {
      try {
        await deleteTask(user.token, taskId);
        setTasks((tasks) => tasks.filter((task) => task.id !== taskId));
      } catch (err) {
        setError(ERROR_MESSAGES.DELETE_MISSION_FAILED_MESSAGE);
      }
    }
  };

  const handleChangeTaskStatus = async (taskId: number) => {
    if (!loading && !user) {
      router.push("/");
      return;
    }

    if (user) {
      try {
        await changeTaskStatus(user.token, taskId);
        setTasks((tasks) =>
          tasks.map((task) =>
            task.id === taskId ? { ...task, is_completed: true } : task
          )
        );
      } catch (err) {
        setError(ERROR_MESSAGES.PUT_MISSION_STATUS_FAILED_MESSAGE);
      }
    }
  };

  const getTaskPosition = (id: UniqueIdentifier) => {
    const numericId = typeof id === "string" ? Number(id) : id;
    return tasks.findIndex((task) => task.id === numericId);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (!over || active.id === over.id) return;

    setTasks((tasks) => {
      const originalPosition = getTaskPosition(active.id);
      const newPosition = getTaskPosition(over.id);

      return arrayMove(tasks, originalPosition, newPosition);
    });
  };

  const sensors = useSensors(useSensor(PointerSensor), useSensor(TouchSensor));

  return (
    <>
      <Navbar />
      <div className="w-full h-full flex flex-col items-center gap-12 mt-2.5">
        <h1>今日のミッション</h1>
        {isLoading ? (
          <p>Loading...</p>
        ) : error ? (
          <p className="text-red-500">{error}</p>
        ) : (
          <DndContext
            sensors={sensors}
            collisionDetection={closestCorners}
            onDragEnd={handleDragEnd}
          >
            <Input onSubmit={handleAddTask} />
            <Column tasks={tasks} />
          </DndContext>
        )}
      </div>
    </>
  );
}
