import {useAxios} from "../../hooks/userAxios";
import {ResultList} from "@/config/ResultModel";

const request = useAxios()


export const QueryActressList = async (data: any) => {
    const res = await request.post({url: `/api/actressList`, data})
    return res as unknown as ResultList
}