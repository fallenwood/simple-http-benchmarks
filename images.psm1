$version="20240216"

$imageBaseName="simple-benchmark"

$port=5000

function Get-TagName {
    param (
        [String]
        $tag
    )
    return "${imageBaseName}:${tag}-$version"
}

function Get-ImageTag {
    param (
        [String]
        $tag
    )
    return "${tag}-${version}"
}

$images = @{ Tag = "dotnet-aot"; Dockerfile="./dotnet/aot/Dockerfile"; Directory="./dotnet/aot"; },
          @{ Tag = "go-gin"; Dockerfile="./go/gin/Dockerfile"; Directory="./go/gin"; },
          @{ Tag = "go-gin-cgo"; Dockerfile="./go/gin/Dockerfile.cgo"; Directory="./go/gin"; }

function Get-Images {
    return $images
}

function Get-ImageBaseName {
    return $imageBaseName
}

function Get-Container {
    return $ENV:BENCH_CONTAINER ?? "docker"
}

function Get-Wrk {
    return $ENV:BENCH_WRK ?? "wrknet";
}

function Get-Port {
    return $port
}

Export-ModuleMember -Function Get-ImageBaseName
Export-ModuleMember -Function Get-ImageTag
Export-ModuleMember -Function Get-TagName
Export-ModuleMember -Function Get-Images
Export-ModuleMember -Function Get-Container
Export-ModuleMember -Function Get-Wrk
Export-ModuleMember -Function Get-Port
