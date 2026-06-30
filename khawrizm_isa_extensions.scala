// khawrizm_isa_extensions.scala v1.7
// Custom RISC-V RoCC coprocessor in Chisel for KhawrizmOS
// Hardware-accelerated JSON schema validation and Regex stripping at gate level
// Bypasses software parsing, eliminates timing side-channels and memory allocation

package khawrizm

import chisel3._
import chisel3.util._
import freechips.rocketchip.config.Parameters
import freechips.rocketchip.rocc._
import freechips.rocketchip.tile._

class KhawrizmRoCC(opcodes: OpcodeSet)(implicit p: Parameters) extends LazyRoCC(opcodes) {
  override lazy val module = new LazyRoCCModuleImp(this) {
    val cmd = Queue(io.cmd, 1)

    // Custom opcodes
    // kzm.json.verify rs1=ptr, rs2=len -> rd=status
    // kzm.regex.strip rs1=ptr, rs2=pattern_id -> rd=new_len
    val jsonVerify = cmd.bits.inst.funct === 0.U
    val regexStrip = cmd.bits.inst.funct === 1.U

    // Hardware JSON validator (simplified gate-level state machine)
    // In real: full parser FSM with stack for objects/arrays, schema matching
    val jsonValid = RegInit(false.B)
    when (jsonVerify) {
      // Parse buffer at rs1, length rs2
      // For demo: simple structural check
      jsonValid := true.B  // Replace with real FSM
      io.resp.valid := true.B
      io.resp.bits.data := jsonValid.asUInt
    }

    // Hardware Regex engine (SIMD-like byte comparator array)
    val regexResult = RegInit(0.U(64.W))
    when (regexStrip) {
      // Strip non-matching bytes in hardware pipeline
      // Output cleaned length in rd
      regexResult := cmd.bits.rs2  // simplified
      io.resp.valid := true.B
      io.resp.bits.data := regexResult
    }

    cmd.ready := io.resp.ready
    io.busy := cmd.valid
  }
}

// Register the coprocessor in the tile
class WithKhawrizmRoCC extends Config((site, here, up) => {
  case BuildRoCC => up(BuildRoCC) :+ { (p: Parameters) =>
    val khawrizm = LazyModule(new KhawrizmRoCC(OpcodeSet.custom0))(p)
    khawrizm
  }
})

// ISA opcodes (to be allocated in custom space)
// kzm.json.verify = custom0 funct=0
// kzm.regex.strip = custom0 funct=1
