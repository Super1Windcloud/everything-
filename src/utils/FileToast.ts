import { toast } from "@steveyuowo/vue-hot-toast";

export function openPathFinishedToast() {
  toast({
    message: "Open path on explorer finished",
    position: "top-center",
    duration: 3000,
    type: "success",
  });
}


export function  copyPathToClipboardToast() {
    toast({
      message: "Path copied to clipboard",
      position: "top-center",
      duration: 3000,
      type: "success",
    })
}


export function copyFileToClipboardToast() {
  toast({
    message: "Copy file to clipboard",
    position: "top-center",
    duration: 3000,
    type: "success",
  })
}




export function deleteFileToast(){
  toast({
    message: "File deleted successfully",
    position: "top-center",
    duration: 3000,
    type: "success",
  })
}
