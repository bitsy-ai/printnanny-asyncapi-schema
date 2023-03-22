
export interface DetectionSettings {
  nats_server_uri: string;
  label_file: string;
  model_file: string;
  nms_threshold: number;
  tensor_batch_size: number;
  tensor_framerate: number;
  tensor_height: number;
  tensor_width: number;
  overlay: boolean;
  graphs: boolean;
}
