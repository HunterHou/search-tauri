import {useAxios} from "../../hooks/userAxios";

const request = useAxios()

export const TypeSizeMap = async () => {
    const res = await request.get({url: '/api/typeSizeMap'})
    return res
}

export const TagSizeMap = async () => {
    const res = await request.get({url: '/api/tagSizeMap'})
    return res
}

export const ScanTime = async () => {
    const res = await request.get({url: '/api/scanTime'})
    return res
}