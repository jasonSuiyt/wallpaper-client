export interface Wallpaper {
    id: number;
    name: string;
    url: string;
    uhd_url: string;
    uhd_file_path: string;
    normal_file_path: string;
    created_date: Date;
    process: number;
    disabled: boolean;
    text: string;
}
