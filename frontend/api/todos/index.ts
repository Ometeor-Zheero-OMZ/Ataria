import axios from "axios";

// Fetch tasks
export const fetchTasks = async (token: string) => {
  try {
    const response = await axios.get("/api/todos", {
      headers: { Authorization: `Bearer ${token}` },
    });
    return response.data;
  } catch (error) {
    throw new Error("Failed to fetch tasks");
  }
};

// Add task
export const addTask = async (
  token: string,
  title: string,
  description: string
) => {
  try {
    await axios.post(
      "/api/todo",
      { title, description },
      {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
      }
    );
  } catch (error) {
    throw new Error("Failed to add task");
  }
};

// Update task
export const updateTask = async (token: string, updatedTask: any) => {
  try {
    await axios.post("/api/todo", updatedTask, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
    });
  } catch (error) {
    throw new Error("Failed to update task");
  }
};

// Delete task
export const deleteTask = async (token: string, taskId: number) => {
  try {
    await axios.delete("/api/todo", {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: { id: taskId },
    });
  } catch (error) {
    throw new Error("Failed to delete task");
  }
};

// Change task status
export const changeTaskStatus = async (token: string, taskId: number) => {
  try {
    await axios.post(
      "/api/todo/complete",
      { id: taskId },
      {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
      }
    );
  } catch (error) {
    throw new Error("Failed to change task status");
  }
};
